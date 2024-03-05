use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
enum Status {
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
    status: Status,
    text: Option<String>,
    url: Option<String>,
    picker: Option<Vec<PickerItem>>,
}

#[derive(Deserialize, Debug)]
struct PickerItem {
    url: String,
    #[serde(rename = "type")]
    item_type: Option<String>,
    thumb: Option<String>,
}
