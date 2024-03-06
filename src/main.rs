mod cobalt;
mod discord;
mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    database::start_db().await?;
    discord::make().await; 
    Ok(())
}
