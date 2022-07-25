use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApkParsedInfo {
    pub package_name: String,
    pub version_code: String,
    pub version_name: String,
    pub min_sdk_version: String,
    pub target_sdk_version: String,
    pub compile_sdk_version: String,
    pub compile_sdk_version_code_name: String,
    pub permissions: Vec<String>,
}

impl ApkParsedInfo {
    pub fn new() -> Self {
        ApkParsedInfo {
            package_name: "".to_owned(),
            version_code: "".to_owned(),
            version_name: "".to_owned(),
            min_sdk_version: "".to_owned(),
            target_sdk_version: "".to_owned(),
            compile_sdk_version: "".to_owned(),
            compile_sdk_version_code_name: "".to_owned(),
            permissions: Vec::new(),
        }
    }
}
