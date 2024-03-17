use crate::database::UserType;
use crate::{cobalt, database};
use anyhow::anyhow;
use serenity::all::{
    Channel, CreateAllowedMentions, CreateAttachment, CreateMessage, MessageReference,
};
use serenity::model::channel::Message;
use serenity::prelude::*;

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

        match msg.channel(&ctx).await.unwrap() {
            Channel::Private(ch) => {
                eprintln!("Private channel");
                let user = database::get_user_type(msg.author.id.get()).await.unwrap();
                match user {
                    UserType::Free(_) => {
                        msg.channel_id
                            .say(&ctx, "You need to be a premium user to use the bot in a private chat.")
                            .await
                            .unwrap();
                    }
                    UserType::Premium(_) => {
                        let links = filter(&msg.content);
                        links.iter().for_each(|l| eprintln!("Link: {}", l));
                        let og_links = filter(&msg.content);
                        let og_links = og_links.iter()
                            .map(|l| self.cobalt_client.get_link(l))
                            .collect::<Vec<_>>();

                        use futures::future;
                        let links = future::join_all(og_links).await
                            .into_iter()
                            .filter_map(Result::ok)
                            .collect::<Vec<_>>();

                        for link in links {
                            let _ = send_pickers(&ctx, &msg, &link).await;
                        }
                    }
                }
            }
            Channel::Guild(ch) => {
                let links = filter(&msg.content);
                links.iter().for_each(|l| eprintln!("Link: {}", l));
                eprintln!("Guild channel");
            }
            _ => {
                eprintln!("Unknown channel type");
            }
        }
    }
}

fn filter(content: &str) -> Vec<&str> {
    use regex::Regex;
    let re = Regex::new(r"(http|https)://([^\s]*)").unwrap();

    re.captures_iter(content)
        .filter_map(|c| c.get(0))
        .map(|c| c.as_str())
        .collect()
}

/// Sends a message with multiple attachments.
/// This is not a webhook message.
/// Used for: Private channels
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

    msg.channel_id
        .send_message(ctx, builder)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
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
