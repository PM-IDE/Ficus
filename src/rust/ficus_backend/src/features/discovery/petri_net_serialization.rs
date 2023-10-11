use crate::features::discovery::petri_net::arc::Arc;
use crate::features::discovery::petri_net::petri_net::PetriNet;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use crate::utils::xml_utils::{write_empty, StartEndElementCookie, XmlWriteError};
use quick_xml::events::{BytesText, Event};
use quick_xml::Writer;
use std::cell::RefCell;
use std::fs;
use std::io::Cursor;

const PNML_TAG_NAME: &'static str = "pnml";
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
    use_names_as_ids: bool,
) -> Result<(), XmlWriteError>
where
    TTransitionData: ToString,
{
    match serialize_to_pnml(net, use_names_as_ids) {
        Ok(content) => match fs::write(save_path, content) {
            Ok(_) => Ok(()),
            Err(error) => Err(XmlWriteError::IOError(error)),
        },
        Err(error) => Err(error),
    }
}

pub fn serialize_to_pnml<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
    use_names_as_ids: bool,
) -> Result<String, XmlWriteError>
where
    TTransitionData: ToString,
{
    let writer = RefCell::new(Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2));

    let pnml_cookie = StartEndElementCookie::new(&writer, PNML_TAG_NAME)?;
    let net_cookie = StartEndElementCookie::new(&writer, NET_TAG_NAME)?;

    let get_place_id = |place: &Place| -> String {
        match use_names_as_ids {
            true => place.name().to_owned(),
            false => place.id().to_string(),
        }
    };

    let get_transition_id = |transition: &Transition<TTransitionData, TArcData>| match use_names_as_ids {
        true => transition.name().to_string(),
        false => transition.id().to_string(),
    };

    let create_arc_name = |from_name: String, to_name: String| format!("[{{{}}}--{{{}}}]", from_name, to_name);

    let mut places = net.all_places();
    places.sort_by(|left, right| left.name().cmp(right.name()));

    for place in places {
        let _ = StartEndElementCookie::new_with_attrs(
            &writer,
            PLACE_TAG_NAME,
            &vec![(ID_ATTR_NAME, get_place_id(place).as_str())],
        )?;
    }

    let mut transitions = net.all_transitions();
    transitions.sort_by(|left, right| left.name().cmp(right.name()));

    for transition in &transitions {
        let cookie = StartEndElementCookie::new_with_attrs(
            &writer,
            TRANSITION_TAG_NAME,
            &vec![(ID_ATTR_NAME, get_transition_id(transition).as_str())],
        );

        if let Some(data) = transition.data() {
            let name = StartEndElementCookie::new(&writer, NAME_TAG_NAME);
            let text = StartEndElementCookie::new(&writer, TEXT_TAG_NAME);

            match writer
                .borrow_mut()
                .write_event(Event::Text(BytesText::new(data.to_string().as_str())))
            {
                Ok(()) => {}
                Err(error) => return Err(XmlWriteError::WriterError(error)),
            };

            drop(text);
            drop(name);
        }

        drop(cookie)
    }

    for transition in &transitions {
        let mut incoming_arcs: Vec<(&Arc<TArcData>, String)> = transition
            .incoming_arcs()
            .iter()
            .map(|arc| {
                (
                    arc,
                    match use_names_as_ids {
                        true => {
                            create_arc_name(get_place_id(net.place(&arc.place_id())), get_transition_id(transition))
                        }
                        false => arc.id().to_string(),
                    },
                )
            })
            .collect();

        incoming_arcs.sort_by(|first, second| first.1.cmp(&second.1));

        for arc in &incoming_arcs {
            StartEndElementCookie::new_with_attrs(
                &writer,
                ARC_TAG_NAME,
                &vec![
                    (ID_ATTR_NAME, arc.1.as_str()),
                    (SOURCE_ATTR_NAME, get_place_id(net.place(&arc.0.place_id())).as_str()),
                    (TARGET_ATTR_NAME, get_transition_id(transition).as_str()),
                ],
            )?;
        }

        let mut outgoing_arcs: Vec<(&Arc<TArcData>, String)> = transition
            .outgoing_arcs()
            .iter()
            .map(|arc| {
                (
                    arc,
                    match use_names_as_ids {
                        true => {
                            create_arc_name(get_transition_id(transition), get_place_id(net.place(&arc.place_id())))
                        }
                        false => arc.id().to_string(),
                    },
                )
            })
            .collect();

        outgoing_arcs.sort_by(|first, second| first.1.cmp(&second.1));

        for arc in outgoing_arcs {
            StartEndElementCookie::new_with_attrs(
                &writer,
                ARC_TAG_NAME,
                &vec![
                    (ID_ATTR_NAME, arc.1.as_str()),
                    (TARGET_ATTR_NAME, get_place_id(net.place(&arc.0.place_id())).as_str()),
                    (SOURCE_ATTR_NAME, get_transition_id(transition).as_str()),
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
