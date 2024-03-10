use serenity::prelude::*;
use serenity::model::webhook::Webhook;
use serenity::builder::ExecuteWebhook;
use anyhow::Result;

pub async fn send_webhook(http: &Context) -> Result<()> {
    let webhook = Webhook::from_url(http, "https://discord.com/api/webhooks/1216073571701362838/fN4HeKacrzZKowS43WbNyI7gEnNZXMnWn52r_KchyUvenZcGQTtHvuF6Z9KvbDmNfRF9").await?;

    let builder = ExecuteWebhook::default()
        .username("Cobalt")
        .content("Hello, world!");

    _ = webhook.execute(http, false, builder).await?;
    Ok(())
}
