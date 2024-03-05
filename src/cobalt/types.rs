use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "stream")]
    Stream,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "rate-limit")]
    RateLimit,
    #[serde(rename = "picker")]
    Picker,
}

#[derive(Serialize, Debug)]
pub struct RequestBody {
    url: String,
}

impl RequestBody {
    pub fn new(url: &str) -> Self {
        RequestBody {
            url: url.to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ResponseBody {
    pub status: Status,
    text: Option<String>,
    pub url: Option<String>,
    pub picker: Option<Vec<PickerItem>>,
}

#[derive(Deserialize, Debug)]
pub struct PickerItem {
    pub url: String,
    #[serde(rename = "type")]
    item_type: Option<String>,
    thumb: Option<String>,
}
