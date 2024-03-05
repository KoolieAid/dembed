mod cobalt;
mod discord;

use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let link = cobalt::get_link("https://vt.tiktok.com/ZSFSqL3qP/").await?;
    println!("{}", link);
    discord::make().await; 
    Ok(())
}
