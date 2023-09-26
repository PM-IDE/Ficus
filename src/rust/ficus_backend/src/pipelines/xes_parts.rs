use crate::pipelines::pipeline_parts::PipelineParts;
use crate::utils::performance::performance_cookie::performance_cookie;
use crate::{
    event_log::xes::{reader::file_xes_log_reader::read_event_log, writer::xes_event_log_writer::write_log},
    utils::user_data::user_data::UserData,
};

use super::{
    errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
    pipelines::PipelinePartFactory,
};

impl PipelineParts {
    pub(super) fn write_log_to_xes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::WRITE_LOG_TO_XES, &|context, _, keys, _| {
            let path = Self::get_user_data(context, &keys.path())?;
            match write_log(&context.concrete(&keys.event_log().key()).unwrap(), path) {
                Ok(()) => Ok(()),
                Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(
                    err.to_string(),
                ))),
            }
        })
    }

    pub(super) fn read_log_from_xes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::READ_LOG_FROM_XES, &|context, infra, keys, _| {
            performance_cookie(Self::READ_LOG_FROM_XES, infra, &mut || {
                let path = Self::get_user_data(context, keys.path())?;

                let log = read_event_log(path);
                if log.is_none() {
                    let message = format!("Failed to read event log from {}", path.as_str());
                    return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)));
                }

                context.put_concrete(keys.event_log().key(), log.unwrap());
                Ok(())
            })
        })
    }
}
