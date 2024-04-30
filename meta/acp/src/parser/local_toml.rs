use serde::Deserialize;
use std::collections::HashMap;

// create a struct for LOCAL.toml
#[derive(Debug, Deserialize)]
pub(crate) struct LocalToml {
    #[serde(rename = "crate", default)]
    pub(crate) crate_map: HashMap<String, toml::Table>,

    #[serde(rename = "app", default)]
    pub app_map: HashMap<String, toml::Table>,
}
