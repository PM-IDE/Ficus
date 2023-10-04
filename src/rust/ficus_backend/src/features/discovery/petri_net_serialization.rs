use crate::features::discovery::petri_net::{Arc, PetriNet, Transition};
use crate::utils::xml_utils::{write_empty, StartEndElementCookie, XmlWriteError};
use quick_xml::Writer;
use std::cell::RefCell;
use std::fs;
use std::io::Cursor;
use quick_xml::events::{BytesText, Event};

const PNML_TAG_NAME: &'static str = "pmnl";
const TRANSITION_TAG_NAME: &'static str = "transition";
const ARC_TAG_NAME: &'static str = "arc";
const PLACE_TAG_NAME: &'static str = "place";
const NET_TAG_NAME: &'static str = "net";
const TEXT_TAG_NAME: &'static str = "text";
const NAME_TAG_NAME: &'static str = "name";

const ID_ATTR_NAME: &'static str = "id";
const SOURCE_ATTR_NAME: &'static str = "source";
const TARGET_ATTR_NAME: &'static str = "target";

pub fn serialize_to_pnml_file<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
    save_path: &str,
) -> Result<(), XmlWriteError> where TTransitionData: ToString {
    match serialize_to_pnml(net) {
        Ok(content) => match fs::write(save_path, content) {
            Ok(_) => Ok(()),
            Err(error) => Err(XmlWriteError::IOError(error)),
        },
        Err(error) => Err(error),
    }
}

pub fn serialize_to_pnml<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
) -> Result<String, XmlWriteError> where TTransitionData: ToString {
    let writer = RefCell::new(Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2));

    let pnml_cookie = StartEndElementCookie::new(&writer, PNML_TAG_NAME)?;
    let net_cookie = StartEndElementCookie::new(&writer, NET_TAG_NAME)?;

    for place in net.non_deleted_places() {
        let _ = StartEndElementCookie::new_with_attrs(
            &writer,
            PLACE_TAG_NAME,
            &vec![(ID_ATTR_NAME, place.id().to_string().as_str())],
        )?;
    }

    for transition in net.all_transitions() {
        let cookie = StartEndElementCookie::new_with_attrs(
            &writer,
            TRANSITION_TAG_NAME,
            &vec![(ID_ATTR_NAME, transition.id().to_string().as_str())],
        );

        if let Some(data) = transition.data() {
            let name = StartEndElementCookie::new(&writer, NAME_TAG_NAME);
            let text = StartEndElementCookie::new(&writer, TEXT_TAG_NAME);

            match writer.borrow_mut().write_event(Event::Text(BytesText::new(data.to_string().as_str()))) {
                Ok(()) => {},
                Err(error) => return Err(XmlWriteError::WriterError(error))
            };

            drop(text);
            drop(name);
        }

        drop(cookie)
    }

    for transition in net.all_transitions() {
        for arc in transition.incoming_arcs() {
            StartEndElementCookie::new_with_attrs(
                &writer,
                ARC_TAG_NAME,
                &vec![
                    (ID_ATTR_NAME, arc.id().to_string().as_str()),
                    (SOURCE_ATTR_NAME, net.place(arc.place_index()).id().to_string().as_str()),
                    (TARGET_ATTR_NAME, transition.id().to_string().as_str()),
                ],
            )?;
        }

        for arc in transition.outgoing_args() {
            StartEndElementCookie::new_with_attrs(
                &writer,
                ARC_TAG_NAME,
                &vec![
                    (ID_ATTR_NAME, arc.id().to_string().as_str()),
                    (TARGET_ATTR_NAME, net.place(arc.place_index()).id().to_string().as_str()),
                    (SOURCE_ATTR_NAME, transition.id().to_string().as_str()),
                ],
            )?;
        }
    }

    drop(net_cookie);
    drop(pnml_cookie);

    let content = writer.borrow().get_ref().get_ref().clone();
    match String::from_utf8(content) {
        Ok(string) => Ok(string),
        Err(error) => Err(XmlWriteError::FromUt8Error(error)),
    }
}
