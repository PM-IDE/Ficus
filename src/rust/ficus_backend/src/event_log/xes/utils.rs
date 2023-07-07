use quick_xml::events::BytesStart;

use super::constants::{KEY_ATTR_NAME, VALUE_ATTR_NAME};

pub struct KeyValuePair<TKey, TValue> {
    pub key: Option<TKey>,
    pub value: Option<TValue>,
}

pub fn extract_key_value(start: &BytesStart) -> KeyValuePair<String, String> {
    let mut key: Option<String> = None;
    let mut value: Option<String> = None;

    for attr in start.attributes() {
        match attr {
            Err(_) => continue,
            Ok(real_attr) => match real_attr.key.0 {
                KEY_ATTR_NAME => match String::from_utf8(real_attr.value.to_owned().to_vec()) {
                    Err(_) => continue,
                    Ok(string) => key = Some(string),
                },
                VALUE_ATTR_NAME => match String::from_utf8(real_attr.value.to_owned().to_vec()) {
                    Err(_) => continue,
                    Ok(string) => value = Some(string),
                },
                _ => continue,
            },
        }
    }

    return KeyValuePair { key, value };
}