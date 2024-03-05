mod cobalt;
mod discord;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    discord::make().await; 
    Ok(())
}
