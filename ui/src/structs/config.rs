use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub app_id: Option<String>,
    pub remember: bool,
    pub presets_dir: Option<String>,
    pub minimize: bool,
}
