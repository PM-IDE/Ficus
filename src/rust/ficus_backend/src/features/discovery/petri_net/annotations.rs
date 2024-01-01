use crate::{event_log::core::event_log::EventLog, features::analysis::event_log_info::DfgInfo};

use super::petri_net::{DefaultPetriNet, PetriNet};

pub fn annotate_with_frequencies(log: &impl EventLog, net: &mut DefaultPetriNet) {}
