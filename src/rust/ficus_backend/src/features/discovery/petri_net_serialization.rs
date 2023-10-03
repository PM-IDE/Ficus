use crate::features::discovery::petri_net::{Arc, PetriNet, Transition};
use crate::utils::xml_utils::{write_empty, StartEndElementCookie, XmlWriteError};
use quick_xml::Writer;
use std::cell::RefCell;
use std::io::Cursor;
use std::io::ErrorKind::InvalidData;

const PNML_TAG_NAME: &'static str = "pmnl";
const TRANSITION_TAG_NAME: &'static str = "transition";
const ARC_TAG_NAME: &'static str = "arc";
const PLACE_TAG_NAME: &'static str = "place";
const NET_TAG_NAME: &'static str = "net";

const ID_ATTR_NAME: &'static str = "id";
const SOURCE_ATTR_NAME: &'static str = "source";
const TARGET_ATTR_NAME: &'static str = "target";

pub fn serialize_to_pnml<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
) -> Result<String, XmlWriteError> {
    let writer = RefCell::new(Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2));

    let _ = StartEndElementCookie::new(&writer, PNML_TAG_NAME)?;
    let _ = StartEndElementCookie::new(&writer, NET_TAG_NAME)?;

    for place in net.non_deleted_places() {
        let _ = StartEndElementCookie::new_with_attrs(
            &writer,
            PLACE_TAG_NAME,
            &vec![(ID_ATTR_NAME, place.id().to_string().as_str())],
        )?;
    }

    for transition in net.all_transitions() {
        let _ = StartEndElementCookie::new_with_attrs(
            &writer,
            TRANSITION_TAG_NAME,
            &vec![(ID_ATTR_NAME, transition.id().to_string().as_str())],
        );
    }

    for transition in net.all_transitions() {
        for arc in transition.incoming_arcs() {
            write_arc_tag(&writer, net, transition, arc)?;
        }
    }

    let content = writer.borrow().get_ref().get_ref().clone();
    match String::from_utf8(content) {
        Ok(string) => Ok(string),
        Err(error) => Err(XmlWriteError::FromUt8Error(error)),
    }
}

fn write_arc_tag<TTransitionData, TArcData>(
    writer: &RefCell<Writer<Cursor<Vec<u8>>>>,
    net: &PetriNet<TTransitionData, TArcData>,
    transition: &Transition<TTransitionData, TArcData>,
    arc: &Arc<TArcData>,
) -> Result<(), XmlWriteError> {
    let _ = StartEndElementCookie::new_with_attrs(
        &writer,
        ARC_TAG_NAME,
        &vec![
            (ID_ATTR_NAME, arc.id().to_string().as_str()),
            (TARGET_ATTR_NAME, net.place(arc.place_index()).id().to_string().as_str()),
            (SOURCE_ATTR_NAME, transition.id().to_string().as_str()),
        ],
    )?;

    let _ = StartEndElementCookie::new_with_attrs(
        &writer,
        ARC_TAG_NAME,
        &vec![
            (ID_ATTR_NAME, arc.id().to_string().as_str()),
            (TARGET_ATTR_NAME, net.place(arc.place_index()).id().to_string().as_str()),
            (SOURCE_ATTR_NAME, transition.id().to_string().as_str()),
        ],
    )?;

    Ok(())
}
