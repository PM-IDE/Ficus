use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Duration, Utc};

use crate::{
    event_log::{
        core::{
            event::{
                event::Event,
                event_hasher::{NameEventHasher, RegexEventHasher},
            },
            event_log::EventLog,
            trace::trace::Trace,
        },
        xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl},
    },
    features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto},
    utils::user_data::user_data::{UserData, UserDataImpl},
};

use super::{
    keys::context_keys::ContextKeys,
    pipelines::{PipelinePartFactory, PipelineParts},
};

impl PipelineParts {
    pub(super) fn create_hashed_event_log(
        config: &UserDataImpl,
        keys: &ContextKeys,
        log: &XesEventLogImpl,
    ) -> Vec<Vec<u64>> {
        match Self::get_context_value(config, keys.event_class_regex()) {
            Ok(regex) => {
                let hasher = RegexEventHasher::new(regex).ok().unwrap();
                log.to_hashes_event_log(&hasher)
            }
            Err(_) => log.to_hashes_event_log(&NameEventHasher::new()),
        }
    }

    pub(super) fn get_event_log_info() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_EVENT_LOG_INFO, &|context, keys, _| {
            let log = Self::get_context_value(context, keys.event_log())?;
            let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
            context.put_concrete(keys.event_log_info().key(), log_info);

            Ok(())
        })
    }

    pub(super) fn get_hashes_event_log() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_HASHES_EVENT_LOG, &|context, keys, config| {
            let log = Self::get_context_value(context, keys.event_log())?;
            let hashes_event_log = Self::create_hashed_event_log(config, keys, log);

            context.put_concrete(keys.hashes_event_log().key(), hashes_event_log);

            Ok(())
        })
    }

    pub(super) fn get_names_event_log() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_NAMES_EVENT_LOG, &|context, keys, _| {
            let log = Self::get_context_value(context, keys.event_log())?;

            let mut result = vec![];
            for trace in log.get_traces() {
                let mut vec = vec![];
                for event in trace.borrow().get_events() {
                    vec.push(event.borrow().get_name().to_string());
                }

                result.push(vec);
            }

            context.put_concrete(keys.names_event_log().key(), result);

            Ok(())
        })
    }

    pub(super) fn use_names_event_log() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::USE_NAMES_EVENT_LOG, &|context, keys, _| {
            let names_log = Self::get_context_value(context, keys.names_event_log())?;
            let mut log = XesEventLogImpl::empty();
            for names_trace in names_log {
                let mut trace = XesTraceImpl::empty();
                let mut date = DateTime::<Utc>::MIN_UTC;

                for name in names_trace {
                    let event = XesEventImpl::new_with_date(name.clone(), date.clone());
                    trace.push(Rc::new(RefCell::new(event)));
                    date = date + Duration::seconds(1);
                }

                log.push(Rc::new(RefCell::new(trace)));
            }

            context.put_concrete::<XesEventLogImpl>(keys.event_log().key(), log);

            Ok(())
        })
    }
}
