use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const HOST: &str = "https://co.wuk.sh";

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
struct RequestBody {
    url: String,
}

#[derive(Deserialize, Debug)]
struct ResponseBody {
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

pub async fn get_link(url: &str) -> Result<String> {
    let body = RequestBody {
        url: url.to_string(),
    };

    let client = Client::new();
    let response = client
        .post(format!("{}/api/json", HOST))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let body: ResponseBody = response.json().await?;
    dbg!(&body);

    // return Ok(body.status);
    todo!("Return link");
}
