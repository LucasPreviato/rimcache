use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ModMetadata {
    pub name:  Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub packageId: Option<String>,
    pub supportedVersions: Option<SupportedVersions>,
}

#[derive(Debug, Deserialize)]
pub struct SupportedVersions {
    #[serde(rename = "li", default)]
    pub versions: Vec<String>,
}