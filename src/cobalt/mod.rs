mod types;

pub use types::{PickerItem, RequestBody, ResponseBody, Status};

use anyhow::Result;
use reqwest::Client;

use dotenv_codegen::dotenv;

const HOST: &str = dotenv!("COBALT_HOST");

#[derive(Debug)]
pub enum ResultType {
    Direct(String),
    Picker(Vec<PickerItem>),
}

pub async fn get_link(url: &str) -> Result<ResultType> {
    let body = RequestBody::new(url);

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

    match body.status {
        Status::Success => {
            if let Some(url) = body.url {
                Ok(ResultType::Direct(url))
            } else {
                Err(anyhow::anyhow!("No URL found"))
            }
        }
        Status::Redirect => {
            if let Some(url) = body.url {
                Ok(ResultType::Direct(url))
            } else {
                Err(anyhow::anyhow!("No URL found"))
            }
        }
        Status::Stream => {
            eprintln!("Stream status not implemented");
            Err(anyhow::anyhow!("Stream status not implemented"))
        }
        Status::Picker => {
            if let Some(pickers) = body.picker {
                Ok(ResultType::Picker(pickers))
            } else {
                Err(anyhow::anyhow!("No pickers found"))
            }
        }
        _ => Err(anyhow::anyhow!("Status error: {:?}", body.status)),
    }
}

