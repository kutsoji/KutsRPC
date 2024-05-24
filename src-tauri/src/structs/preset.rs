use crate::structs::Activity;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Preset {
    pub id: i64,
    pub name: String,
    pub picture: Option<String>,
    pub activity: Activity,
    pub path: String,
}
