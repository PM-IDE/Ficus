use crate::test_core::gold_based_test::execute_test_with_gold;
use crate::test_core::simple_events_logs_provider::{
    create_alpha_plus_plus_nfc_test_log9, create_alpha_sharp_test_log2, create_simple_event_log3,
};
use crate::test_core::test_paths::get_serialized_petri_nets_gold_path;
use ficus_backend::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use ficus_backend::features::discovery::alpha::alpha::discover_petri_net_alpha;
use ficus_backend::features::discovery::alpha::alpha_plus_plus_nfc::alpha_plus_plus_nfc::discover_petri_net_alpha_plus_plus_nfc;
use ficus_backend::features::discovery::alpha::alpha_sharp::discover_petri_net_alpha_sharp;
use ficus_backend::features::discovery::alpha::providers::alpha_provider::DefaultAlphaRelationsProvider;
use ficus_backend::features::discovery::petri_net::pnml_serialization::serialize_to_pnml;

#[test]
pub fn alpha_simple_test_1() {
    execute_test_with_gold(get_serialized_petri_nets_gold_path("alpha_simple_test_1"), || {
        let log = create_simple_event_log3();
        let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
        let provider = DefaultAlphaRelationsProvider::new(&info);

        serialize_to_pnml(&discover_petri_net_alpha(&provider), true).ok().unwrap()
    })
}

#[test]
pub fn alpha_sharp_test() {
    let log = create_alpha_sharp_test_log2();
    discover_petri_net_alpha_sharp(&log);
}

#[test]
pub fn alpha_plus_plus_nfc_test() {
    let log = create_alpha_plus_plus_nfc_test_log9();
    discover_petri_net_alpha_plus_plus_nfc(&log);
}
