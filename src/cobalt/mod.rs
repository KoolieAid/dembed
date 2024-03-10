mod types;

pub use types::{PickerItem, RequestBody, ResponseBody, Status};

use anyhow::Result;
use reqwest::Client;

use dotenv_codegen::dotenv;

const HOST: &str = dotenv!("COBALT_HOST");

#[derive(Debug)]
pub enum ResultCount {
    Single(String),
    Multiple(Vec<PickerItem>),
}

#[derive(Debug)]
pub struct Cobalt {
    client: Client,
}

#[allow(dead_code)]
impl Cobalt {
    pub fn new() -> Cobalt {
        Cobalt {
            client: Client::new(),
        }
    }

    pub fn with(client: Client) -> Cobalt {
        Cobalt { client }
    }

    pub async fn get_link(&self, url: &str) -> Result<ResultCount> {
        let body = RequestBody::new(url);

        let client = &self.client;
        let response = client
            .post(format!("{}/api/json", HOST))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let body: ResponseBody = response.json().await?;
        dbg!(&body.status);

        let url: String = body.url.unwrap_or_default();

        match body.status {
            Status::Success => Ok(ResultCount::Single(url)),
            Status::Redirect => Ok(ResultCount::Single(url)),
            Status::Stream => Ok(ResultCount::Single(url)),
            Status::Picker => {
                if let Some(pickers) = body.picker {
                    Ok(ResultCount::Multiple(pickers))
                } else {
                    Err(anyhow::anyhow!("No pickers found"))
                }
            }
            _ => Err(anyhow::anyhow!("E:{}", body.text.unwrap_or_default())),
        }
    }

    pub async fn get_bytes(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        let response = self.client.get(url).send().await?;
        Ok(response.bytes().await?.to_vec())
    }
}

impl Default for Cobalt {
    fn default() -> Self {
        Cobalt::new()
    }
}
