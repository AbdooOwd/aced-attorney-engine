use serde_json::from_str;
use std::fs;
use crate::types::TextboxData;

// TODO: for now, one JSON file will only contain one textboxData


pub fn import_data(path: &str) -> TextboxData {
    let file_content = fs::read_to_string(path).expect("Couldn't read data file");
    let json_parsed: TextboxData = from_str(file_content.as_str()).expect("Couldn't Parse JSON");
    json_parsed
}
