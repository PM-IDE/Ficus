use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct XesEventLogExtension {
    pub name: String,
    pub prefix: String,
    pub uri: String
}

#[derive(Debug, Clone)]
pub struct XesGlobal {
    pub scope: String,
    pub default_values: HashMap<String, String>
}

#[derive(Debug, Clone)]
pub struct XesClassifier {
    pub name: String,
    pub keys: Vec<String>
}