use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Activity {
    pub state: Option<String>,
    pub details: Option<String>,
    pub timestamp: Option<String>,
    pub large_img_key: Option<String>,
    pub large_img_txt: Option<String>,
    pub small_img_key: Option<String>,
    pub small_img_txt: Option<String>,
    pub first_btn_txt: Option<String>,
    pub first_btn_url: Option<String>,
    pub second_btn_txt: Option<String>,
    pub second_btn_url: Option<String>,
}
