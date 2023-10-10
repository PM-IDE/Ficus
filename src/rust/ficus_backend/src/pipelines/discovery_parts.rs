use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::alpha::discover_petri_net_alpha;
use crate::features::discovery::petri_net_serialization::serialize_to_pnml_file;
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::PipelinePartFactory;
use crate::utils::user_data::user_data::UserData;

impl PipelineParts {
    pub(super) fn discover_petri_net_alpha() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA, &|context, infra, keys, config| {
            let log = Self::get_user_data(context, keys.event_log())?;
            let event_log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
            let discovered_net = discover_petri_net_alpha(&event_log_info);

            context.put_concrete(keys.petri_net().key(), discovered_net);

            Ok(())
        })
    }

    pub(super) fn serialize_petri_net() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::SERIALIZE_PETRI_NET, &|context, infra, keys, config| {
            let petri_net = Self::get_user_data(context, keys.petri_net())?;
            let save_path = Self::get_user_data(config, keys.path())?;
            let use_names_as_ids = *Self::get_user_data(config, keys.pnml_use_names_as_ids())?;

            match serialize_to_pnml_file(petri_net, save_path, use_names_as_ids) {
                Ok(_) => Ok(()),
                Err(error) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(
                    error.to_string(),
                ))),
            }
        })
    }
}
