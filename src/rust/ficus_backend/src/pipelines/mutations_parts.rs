use crate::features::mutations::mutations::add_artificial_start_end_activities;
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::PipelinePartFactory;

impl PipelineParts {
    pub(super) fn add_artificial_start_end_events() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::ADD_ARTIFICIAL_START_END_EVENTS, &|context, _, keys, _| {
            let log = Self::get_user_data_mut(context, keys.event_log())?;
            add_artificial_start_end_activities(log);

            Ok(())
        })
    }
}
