use std::{any::Any, str::FromStr, sync::Arc};

use nameof::name_of_type;

use crate::{
    features::analysis::{
        event_log_info::EventLogInfo,
        patterns::{
            contexts::PatternsDiscoveryStrategy, repeat_sets::SubArrayWithTraceIndex,
            tandem_arrays::SubArrayInTraceInfo,
        },
    },
    ficus_proto::{
        grpc_context_value::ContextValue, GrpcColor, GrpcColoredRectangle, GrpcColorsEventLog, GrpcColorsTrace,
        GrpcContextKeyValue, GrpcContextValue, GrpcEventLogInfo, GrpcEventLogTraceSubArraysContextValue,
        GrpcHashesEventLog, GrpcHashesEventLogContextValue, GrpcHashesLogTrace, GrpcNamesEventLog,
        GrpcNamesEventLogContextValue, GrpcNamesTrace, GrpcSubArrayWithTraceIndex,
        GrpcSubArraysWithTraceIndexContextValue, GrpcTraceSubArray, GrpcTraceSubArrays,
    },
    pipelines::{
        aliases::ColorsEventLog,
        context::{LogMessageHandler, PipelineContext},
        keys::{context_key::ContextKey, context_keys::ContextKeys},
        pipelines::PipelineParts,
    },
    utils::{
        colors::{Color, ColoredRectangle},
        user_data::{keys::Key, user_data::UserData},
    },
};

pub(super) fn create_initial_context<'a>(
    values: &Vec<GrpcContextKeyValue>,
    keys: &Arc<Box<ContextKeys>>,
    pipeline_parts: &'a PipelineParts,
    log_message_handler: Arc<Box<dyn LogMessageHandler>>,
) -> PipelineContext<'a> {
    let mut context = PipelineContext::new_with_logging(pipeline_parts, log_message_handler);

    for value in values {
        let key = keys.find_key(&value.key.as_ref().unwrap().name).unwrap();
        let value = value.value.as_ref().unwrap().context_value.as_ref().unwrap();
        put_into_user_data(key.key(), value, &mut context);
    }

    context
}

pub(super) fn put_into_user_data(key: &dyn Key, value: &ContextValue, user_data: &mut impl UserData) {
    match value {
        ContextValue::String(string) => user_data.put_any::<String>(key, string.clone()),
        ContextValue::HashesLog(_) => todo!(),
        ContextValue::NamesLog(grpc_log) => put_names_log_to_context(key, grpc_log, user_data),
        ContextValue::Uint32(number) => user_data.put_any::<u32>(key, number.clone()),
        ContextValue::TracesSubArrays(_) => todo!(),
        ContextValue::TraceIndexSubArrays(_) => todo!(),
        ContextValue::Bool(bool) => user_data.put_any::<bool>(key, bool.clone()),
        ContextValue::XesEventLog(grpc_log) => put_names_log_to_context(key, grpc_log, user_data),
        ContextValue::ColorsLog(_) => {}
        ContextValue::Enum(grpc_enum) => {
            let enum_name = &grpc_enum.enum_type;
            if enum_name == name_of_type!(PatternsDiscoveryStrategy) {
                if let Ok(strategy) = PatternsDiscoveryStrategy::from_str(&grpc_enum.value) {
                    user_data.put_any::<PatternsDiscoveryStrategy>(key, strategy);
                }
            }
        }
        ContextValue::EventLogInfo(_) => todo!(),
        ContextValue::Strings(strings) => user_data.put_any::<Vec<String>>(key, strings.strings.clone()),
    }
}

fn put_names_log_to_context(key: &dyn Key, grpc_log: &GrpcNamesEventLogContextValue, user_data: &mut impl UserData) {
    let grpc_log = grpc_log.log.as_ref().unwrap();
    let mut names_log = vec![];
    for grpc_trace in &grpc_log.traces {
        let mut trace = vec![];
        for grpc_event in &grpc_trace.events {
            trace.push(grpc_event.clone());
        }

        names_log.push(trace);
    }

    user_data.put_any::<Vec<Vec<String>>>(key, names_log);
}

pub fn convert_to_grpc_context_value(
    key: &dyn ContextKey,
    value: &dyn Any,
    keys: &ContextKeys,
) -> Option<GrpcContextValue> {
    if keys.is_path(key) {
        try_convert_to_string_context_value(value)
    } else if keys.is_hashes_event_log(key) {
        try_convert_to_hashes_event_log(value)
    } else if keys.is_names_event_log(key) {
        try_convert_to_names_event_log(value)
    } else if keys.is_patterns(key) {
        try_convert_to_grpc_traces_sub_arrays(value)
    } else if keys.is_repeat_sets(key) {
        try_convert_to_grpc_sub_arrays_with_index(value)
    } else if keys.is_colors_event_log(key) {
        try_convert_to_grpc_colors_event_log(value)
    } else if keys.is_event_log_info(key) {
        try_convert_to_grpc_event_log_info(value)
    } else {
        None
    }
}

fn try_convert_to_string_context_value(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<String>() {
        None
    } else {
        Some(GrpcContextValue {
            context_value: Some(ContextValue::String(value.downcast_ref::<String>().unwrap().clone())),
        })
    }
}

fn try_convert_to_hashes_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<Vec<u64>>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<Vec<u64>>>().unwrap();
        let mut traces = vec![];
        for trace in vec {
            let mut events = vec![];
            for event in trace {
                events.push(*event);
            }

            traces.push(GrpcHashesLogTrace { events });
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::HashesLog(GrpcHashesEventLogContextValue {
                log: Some(GrpcHashesEventLog { traces }),
            })),
        })
    }
}

fn try_convert_to_names_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<Vec<String>>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<Vec<String>>>().unwrap();
        let mut traces = vec![];
        for trace in vec {
            let mut events = vec![];
            for event in trace {
                events.push(event.clone());
            }

            traces.push(GrpcNamesTrace { events });
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::NamesLog(GrpcNamesEventLogContextValue {
                log: Some(GrpcNamesEventLog { traces }),
            })),
        })
    }
}

fn try_convert_to_grpc_traces_sub_arrays(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<Vec<SubArrayInTraceInfo>>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<Vec<SubArrayInTraceInfo>>>().unwrap();
        let mut traces = vec![];
        for trace in vec {
            let mut sub_arrays = vec![];
            for array in trace {
                sub_arrays.push(GrpcTraceSubArray {
                    start: array.start_index as u32,
                    end: (array.start_index + array.length) as u32,
                })
            }

            traces.push(GrpcTraceSubArrays { sub_arrays })
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::TracesSubArrays(GrpcEventLogTraceSubArraysContextValue {
                traces_sub_arrays: traces,
            })),
        })
    }
}

fn try_convert_to_grpc_sub_arrays_with_index(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<SubArrayWithTraceIndex>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<SubArrayWithTraceIndex>>().unwrap();
        let mut sub_arrays = vec![];

        for array in vec {
            sub_arrays.push(GrpcSubArrayWithTraceIndex {
                sub_array: Some(GrpcTraceSubArray {
                    start: array.sub_array.start_index as u32,
                    end: (array.sub_array.start_index + array.sub_array.length) as u32,
                }),
                trace_index: array.trace_index as u32,
            })
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::TraceIndexSubArrays(
                GrpcSubArraysWithTraceIndexContextValue { sub_arrays },
            )),
        })
    }
}

fn try_convert_to_grpc_colors_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<ColorsEventLog>() {
        None
    } else {
        let colors_log = value.downcast_ref::<ColorsEventLog>().unwrap();
        let mut grpc_traces = vec![];

        for trace in colors_log {
            let mut grpc_trace = vec![];
            for colored_rect in trace {
                grpc_trace.push(convert_to_grpc_colored_rect(colored_rect))
            }

            grpc_traces.push(GrpcColorsTrace {
                event_colors: grpc_trace,
            })
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::ColorsLog(GrpcColorsEventLog { traces: grpc_traces })),
        })
    }
}

fn convert_to_grpc_colored_rect(colored_rect: &ColoredRectangle) -> GrpcColoredRectangle {
    GrpcColoredRectangle {
        color: Some(convert_to_grpc_color(&colored_rect.color())),
        start_index: colored_rect.start_pos() as u32,
        length: colored_rect.len() as u32,
        name: colored_rect.name().to_owned(),
    }
}

fn convert_to_grpc_color(color: &Color) -> GrpcColor {
    GrpcColor {
        red: color.red() as u32,
        green: color.green() as u32,
        blue: color.blue() as u32,
    }
}

fn try_convert_to_grpc_event_log_info(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<EventLogInfo>() {
        None
    } else {
        let log_info = value.downcast_ref::<EventLogInfo>().unwrap();
        Some(GrpcContextValue {
            context_value: Some(ContextValue::EventLogInfo(GrpcEventLogInfo {
                events_count: log_info.get_events_count() as u32,
                traces_count: log_info.traces_count() as u32,
                event_classes_count: log_info.event_classes_count() as u32,
            })),
        })
    }
}
