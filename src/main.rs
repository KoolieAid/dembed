mod cobalt;
mod discord;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let link = cobalt::get_link("https://vt.tiktok.com/ZSFSqL3qP/").await?;
    println!("{}", link);
    Ok(())
}
