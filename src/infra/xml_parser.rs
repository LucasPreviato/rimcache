use crate::domain::mod_metadata::ModMetadata;
use quick_xml::de::from_str;
use std::fs;


pub fn parse_about_xml(path: &str) -> Option<ModMetadata> {
    let content = fs::read_to_string(path).ok()?;
    from_str::<ModMetadata>(&content).ok()
}