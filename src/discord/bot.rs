use crate::cobalt;
use anyhow::anyhow;
use serenity::all::{
    CreateAllowedMentions, CreateAttachment, CreateMessage, CreateWebhook, GuildChannel,
    MessageReference, Webhook,
};
use serenity::model::channel::Message;
use serenity::prelude::*;
use url::Url;

mod types;

pub struct Handler {
    pub cobalt_client: cobalt::Cobalt,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        eprintln!("User Message: {:?}", msg.content);
        let Ok(url) = Url::parse(&msg.content) else {
            eprintln!("Error parsing URL");
            return;
        };

        eprintln!("Host: {:?}", url.host_str());

        if url.host_str().is_none() {
            eprintln!("No host found");
            return;
        }

        use cobalt::ResultCount;
        let link = self.cobalt_client.get_link(&msg.content).await;
        _ = match link {
            Ok(ResultCount::Single(url)) => send_single_webhook(&ctx, &msg, &url).await,
            Ok(ResultCount::Multiple(pickers)) => send_pickers(&ctx, &msg, &pickers).await,
            Err(why) => {
                eprintln!("Error getting link: {:?}", why);
                Err(why)
            }
        };
    }
}

async fn get_any_webhook(ctx: &Context, channel: &GuildChannel) -> anyhow::Result<Webhook> {
    use serenity::model::webhook::WebhookType;
    let webhooks = channel
        .webhooks(ctx)
        .await?
        .into_iter()
        .filter(|w| w.kind == WebhookType::Incoming)
        .collect::<Vec<_>>();

    if !webhooks.is_empty() {
        return Ok(webhooks.into_iter().next().unwrap());
    }

    let wb_builder = CreateWebhook::new("Dembed").audit_log_reason("Creating webhook for Dembed");

    let webhook = channel.create_webhook(ctx, wb_builder).await?;
    Ok(webhook)
}

async fn send_single_webhook(ctx: &Context, msg: &Message, url: &str) -> anyhow::Result<()> {
    use serenity::builder::ExecuteWebhook;

    use serenity::model::channel::Channel;
    let channel = msg.channel(ctx).await?;
    let webhook = match channel {
        Channel::Guild(channel) => get_any_webhook(ctx, &channel).await?,
        Channel::Private(_channel) => todo!("Handle private channel"),
        _ => todo!("Handle new type of channel"),
    };

    let builder = ExecuteWebhook::default()
        .add_file(convert_to_attachment(url).await?)
        .username(msg.member(ctx).await?.display_name())
        .avatar_url(msg.author.avatar_url().unwrap_or_default());

    webhook.execute(ctx, false, builder).await?;
    msg.delete(ctx).await?;
    Ok(())
}

async fn send_msg(ctx: &Context, msg: &Message, direct_link: &str) {
    let builder = CreateMessage::default()
        .reference_message(MessageReference::from(msg))
        .allowed_mentions(CreateAllowedMentions::default().replied_user(false))
        .add_file(convert_to_attachment(direct_link).await.unwrap());

    if let Err(e) = msg.channel_id.send_message(ctx, builder).await {
        eprintln!("Error sending message {:?}", e);
    }
}

async fn send_pickers(
    ctx: &Context,
    msg: &Message,
    pickers: &[cobalt::PickerItem],
) -> anyhow::Result<()> {
    use futures::future::join_all;

    let futures = pickers
        .iter()
        .map(|item| convert_to_attachment(&item.url))
        .collect::<Vec<_>>();
    let results = join_all(futures).await;
    let results = results
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let builder = CreateMessage::default()
        .add_files(results.into_iter())
        .reference_message(MessageReference::from(msg))
        .allowed_mentions(CreateAllowedMentions::default().replied_user(false));

    if let Err(e) = msg.channel_id.send_message(ctx, builder).await {
        eprintln!("Error sending message {:?}", e);
    }
    Ok(())
}

async fn convert_to_attachment(item: &str) -> anyhow::Result<CreateAttachment> {
    let response = reqwest::get(item).await?;
    let content_type = response
        .headers()
        .get("content-type")
        .ok_or(anyhow!("Content-Type header is missing"))?;
    let mut name = String::from("item");
    match content_type.to_str()? {
        "video/mp4" => name.push_str(".mp4"),
        "image/jpeg" => name.push_str(".jpg"),
        "image/png" => name.push_str(".png"),
        _ => return Err(anyhow!("Unsupported content type")),
    }
    let bytes = response.bytes().await?;
    Ok(CreateAttachment::bytes(bytes, name))
}
