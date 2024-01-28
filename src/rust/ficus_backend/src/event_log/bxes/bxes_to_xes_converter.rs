use std::rc::Rc;

use bxes::{
    models::{
        BxesClassifier, BxesEvent, BxesEventLog, BxesEventLogMetadata, BxesExtension, BxesGlobal, BxesGlobalKind, BxesTraceVariant,
        BxesValue,
    },
    writer::{errors::BxesWriteError, single_file_bxes_writer::write_bxes},
};

use crate::event_log::{
    core::{
        event::event::{Event, EventPayloadValue},
        event_log::EventLog,
        trace::trace::Trace,
    },
    xes::{constants::EVENT_TAG_NAME_STR, shared::XesEventLogExtension, xes_event::XesEventImpl, xes_event_log::XesEventLogImpl},
};

use super::conversions::{convert_xes_to_bxes_lifecycle, payload_value_to_bxes_value};

pub fn write_event_log_to_bxes(log: &XesEventLogImpl, path: &str) -> Result<(), BxesWriteError> {
    let bxes_log = BxesEventLog {
        metadata: BxesEventLogMetadata {
            classifiers: Some(create_bxes_classifiers(log)),
            extensions: Some(create_bxes_extensions(log)),
            globals: Some(create_bxes_globals(log)),
            properties: Some(create_bxes_properties(log)),
        },
        variants: create_bxes_traces(log),
        version: 1,
    };

    write_bxes(path, &bxes_log)
}

fn create_bxes_traces(log: &XesEventLogImpl) -> Vec<BxesTraceVariant> {
    log.traces()
        .iter()
        .map(|trace| BxesTraceVariant {
            traces_count: 1,
            metadata: vec![],
            events: trace
                .borrow()
                .events()
                .iter()
                .map(|event| create_bxes_event(log, &event.borrow()))
                .collect(),
        })
        .collect()
}

fn create_bxes_event(log: &XesEventLogImpl, event: &XesEventImpl) -> BxesEvent {
    BxesEvent {
        name: Rc::new(Box::new(BxesValue::String(event.name_pointer().clone()))),
        lifecycle: match event.lifecycle() {
            None => bxes::models::Lifecycle::Standard(bxes::models::StandardLifecycle::Unspecified),
            Some(lifecycle) => convert_xes_to_bxes_lifecycle(lifecycle),
        },
        timestamp: event.timestamp().timestamp_nanos(),
        attributes: Some(
            event
                .ordered_payload()
                .iter()
                .filter(|kv| is_not_default_attribute(log, kv))
                .map(|kv| kv_pair_to_bxes_pair(kv))
                .collect(),
        ),
    }
}

fn is_not_default_attribute(log: &XesEventLogImpl, kv: &(&String, &EventPayloadValue)) -> bool {
    if let Some(event_globals) = log.globals_map().get(EVENT_TAG_NAME_STR) {
        if let Some(default_value) = event_globals.get(kv.0) {
            default_value != kv.1
        } else {
            true
        }
    } else {
        true
    }
}

fn create_bxes_classifiers(log: &XesEventLogImpl) -> Vec<BxesClassifier> {
    log.classifiers()
        .iter()
        .map(|c| BxesClassifier {
            keys: c
                .keys
                .iter()
                .map(|x| Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(x.to_owned()))))))
                .collect(),
            name: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(c.name.to_owned()))))),
        })
        .collect()
}

fn create_bxes_extensions(log: &XesEventLogImpl) -> Vec<BxesExtension> {
    log.extensions().iter().map(|e| convert_to_bxes_extension(e)).collect()
}

fn convert_to_bxes_extension(e: &XesEventLogExtension) -> BxesExtension {
    BxesExtension {
        name: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(e.name.to_owned()))))),
        prefix: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(e.prefix.to_owned()))))),
        uri: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(e.uri.to_owned()))))),
    }
}

fn create_bxes_globals(log: &XesEventLogImpl) -> Vec<BxesGlobal> {
    log.ordered_globals()
        .iter()
        .map(|g| BxesGlobal {
            entity_kind: parse_entity_kind(g.0.as_str()),
            globals: g.1.iter().map(|kv| convert_to_bxes_global_attribute(kv)).collect(),
        })
        .collect()
}

fn parse_entity_kind(string: &str) -> BxesGlobalKind {
    match string {
        "event" => BxesGlobalKind::Event,
        "trace" => BxesGlobalKind::Trace,
        "log" => BxesGlobalKind::Log,
        _ => panic!(),
    }
}

fn convert_to_bxes_global_attribute(kv: &(&String, &EventPayloadValue)) -> (Rc<Box<BxesValue>>, Rc<Box<BxesValue>>) {
    let key = Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(kv.0.to_owned())))));
    let value = Rc::new(Box::new(payload_value_to_bxes_value(kv.1)));

    (key, value)
}

fn create_bxes_properties(log: &XesEventLogImpl) -> Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)> {
    log.properties_map()
        .iter()
        .map(|kv| kv_pair_to_bxes_pair(&(&kv.name, &kv.value)))
        .collect()
}

fn kv_pair_to_bxes_pair(kv: &(&String, &EventPayloadValue)) -> (Rc<Box<BxesValue>>, Rc<Box<BxesValue>>) {
    let bxes_value = payload_value_to_bxes_value(kv.1);
    let key = Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(kv.0.to_owned())))));

    (key, Rc::new(Box::new(bxes_value)))
}
