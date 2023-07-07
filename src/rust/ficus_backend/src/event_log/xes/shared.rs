use std::collections::HashMap;

pub struct XesEventLogExtension {
    pub name: String,
    pub prefix: String,
    pub uri: String
}

pub struct XesGlobal {
    pub scope: String,
    pub default_values: HashMap<String, String>
}

pub struct XesClassifier {
    pub name: String,
    pub keys: Vec<String>
}