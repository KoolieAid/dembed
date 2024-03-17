mod types;

pub use types::{PickerItem, RequestBody, ResponseBody, Status};

use anyhow::{anyhow, Result};
use reqwest::Client;

use dotenv_codegen::dotenv;

const HOST: &str = dotenv!("COBALT_HOST");

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

    pub async fn get_link(&self, url: &str) -> Result<Vec<PickerItem>> {
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
            Status::Success => Ok(vec![url.into()]),
            Status::Redirect => Ok(vec![url.into()]),
            Status::Stream => Ok(vec![url.into()]),
            Status::Picker => body.picker.ok_or(anyhow!("No picker items found")),
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
