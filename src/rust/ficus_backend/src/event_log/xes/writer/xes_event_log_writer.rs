use std::{io::Cursor, fs};

use quick_xml::{Writer, events::{BytesStart, Event, BytesEnd}};

use crate::event_log::xes::{xes_event_log::XesEventLogImpl, constants::*};

pub fn write_log(log: &XesEventLogImpl, save_path: &str) {
    match serialize_log(log) {
        Some(content) => _ = fs::write(save_path, content),
        None => {},
    }
}

fn serialize_log(log: &XesEventLogImpl) -> Option<String> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    
    {
        let log_cookie = StartEndElementCookie::new(&mut writer, LOG_TAG_NAME_STR);
    }
    
    match String::from_utf8(writer.into_inner().into_inner()) {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

struct StartEndElementCookie<'a> {
    tag_name: &'static str,
    writer: &'a mut Writer<Cursor<Vec<u8>>>,
}

impl<'a> Drop for StartEndElementCookie<'a> {
    fn drop(&mut self) {
        assert!(self.writer.write_event(Event::End(BytesEnd::new(self.tag_name))).is_ok());
    }
}

impl<'a> StartEndElementCookie<'a> {
    fn new(writer: &'a mut Writer<Cursor<Vec<u8>>>, tag_name: &'static str) -> StartEndElementCookie<'a> {
        writer.write_event(Event::Start(BytesStart::new(tag_name)));
        StartEndElementCookie { tag_name, writer }
    }
}