mod types;

use types::{RequestBody, ResponseBody};
use anyhow::Result;
use reqwest::Client;

use dotenv_codegen::dotenv;

const HOST: &str = dotenv!("COBALT_HOST");

pub async fn get_link(url: &str) -> Result<String> {
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

    // return Ok(body.status);
    todo!("Return link");
}
