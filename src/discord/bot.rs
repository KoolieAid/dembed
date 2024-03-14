use crate::cobalt;
use anyhow::anyhow;
use serenity::all::{Channel, CreateAllowedMentions, CreateAttachment, CreateMessage, MessageReference};
use serenity::model::channel::Message;
use serenity::prelude::*;
use url::Url;

mod types;
mod webhook;

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

        match msg.channel(&ctx).await.unwrap() {
            Channel::Private(ch) => {
                eprintln!("Private channel");
            },
            Channel::Guild(ch) => {
                eprintln!("Guild channel");
            },
            _ => {
                eprintln!("Unknown channel type");
            },
        }

        use cobalt::ResultCount;
        let link = self.cobalt_client.get_link(&msg.content).await;
        _ = match link {
            Ok(ResultCount::Single(url)) => send_msg(&ctx, &msg, &url).await,
            Ok(ResultCount::Multiple(pickers)) => send_pickers(&ctx, &msg, &pickers).await,
            Err(why) => {
                eprintln!("Error getting link: {:?}", why);
                Err(why)
            }
        };
    }
}

/// Sends a message with 1 attachment.
/// This is not a webhook message.
async fn send_msg(ctx: &Context, msg: &Message, direct_link: &str) -> anyhow::Result<()> {
    let builder = CreateMessage::default()
        .reference_message(MessageReference::from(msg))
        .allowed_mentions(CreateAllowedMentions::default().replied_user(false))
        .add_file(convert_to_attachment(direct_link).await.unwrap());

    msg.channel_id
        .send_message(ctx, builder)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}

/// Sends a message with multiple attachments.
/// This is not a webhook message.
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
