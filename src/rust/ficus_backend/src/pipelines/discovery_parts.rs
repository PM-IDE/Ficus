use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::alpha::{discover_petri_net_alpha, discover_petri_net_alpha_plus};
use crate::features::discovery::alpha::alpha_plus_plus_nfc::alpha_plus_plus_nfc::discover_petri_net_alpha_plus_plus_nfc;
use crate::features::discovery::petri_net::pnml_serialization::serialize_to_pnml_file;
use crate::pipelines::context::PipelineContext;
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::ContextKeys;
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
                Err(error) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(error.to_string()))),
            }
        })
    }

    pub(super) fn discover_petri_net_alpha_plus() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA_PLUS, &|context, infra, keys, config| {
            Self::do_discover_petri_net_alpha_plus(context, keys, false)
        })
    }

    fn do_discover_petri_net_alpha_plus(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        alpha_plus_plus: bool,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_user_data(context, keys.event_log())?;

        let discovered_net = discover_petri_net_alpha_plus(log, alpha_plus_plus);

        context.put_concrete(keys.petri_net().key(), discovered_net);

        Ok(())
    }

    pub(super) fn discover_petri_net_alpha_plus_plus() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA_PLUS_PLUS, &|context, infra, keys, config| {
            Self::do_discover_petri_net_alpha_plus(context, keys, true)
        })
    }

    pub(super) fn discover_petri_net_alpha_plus_plus_nfc() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA_PLUS_PLUS_NFC, &|context, _, keys, config| {
            let log = Self::get_user_data(context, keys.event_log())?;
            let discovered_petri_net = discover_petri_net_alpha_plus_plus_nfc(log);
            context.put_concrete(keys.petri_net().key(), discovered_petri_net);

            Ok(())
        })
    }
}
