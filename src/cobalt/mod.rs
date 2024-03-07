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

#[derive(Debug)]
struct Cobalt {
    http: reqwest::Client;
}

impl Cobalt {
    pub fn with(client: Client) -> Cobalt {
        Cobalt {
            client,
        }
    }

    pub fn new() -> Cobalt {
        Cobalt {
            client: Client::new(),
        }
    }

    pub async fn get_link(&self, url: &str) -> Result<ResultType> {
        let body = RequestBody::new(url);

        let client = &self.http;
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
            Status::Success => Ok(ResultType::Direct(url)),
            Status::Redirect => Ok(ResultType::Direct(url)),
            Status::Stream => Ok(ResultType::Direct(url)),
            Status::Picker => {
                if let Some(pickers) = body.picker {
                    Ok(ResultType::Picker(pickers))
                } else {
                    Err(anyhow::anyhow!("No pickers found"))
                }
            }
            _ => Err(anyhow::anyhow!("E:{}", body.text.unwrap_or_default())),
        }
    }

    pub async get_bytes(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        let response = self.http.get(url).send().await?;
        response.bytes().await?
    }
}

impl Default for Cobalt {
    fn default() -> Self {
        Cobalt::new()
    }
}
