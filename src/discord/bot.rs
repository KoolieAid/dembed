use crate::cobalt;
use serenity::all::{CreateAttachment, CreateMessage, MessageReference};
use serenity::model::channel::Message;
use serenity::prelude::*;
use url::Url;

pub struct Handler {
    pub cobalt_client: reqwest::Client,
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

        match url.host_str() {
            None => return,
            Some(_) => {
                let link = cobalt::get_link(&msg.content).await;
                match link {
                    Ok(cobalt::ResultType::Direct(url)) => {
                        send_msg(&ctx, &msg, &url).await;
                    }
                    Ok(cobalt::ResultType::Picker(pickers)) => {
                        send_pickers(&ctx, &msg, &pickers).await;
                    }
                    Err(why) => {
                        eprintln!("Error getting link: {:?}", why);
                    }
                }
            } // Some(_) => return,
        }
    }
}

async fn send_msg(ctx: &Context, msg: &Message, content: &str) {
    if let Err(why) = msg.reply(&ctx, content).await {
        eprintln!("Error sending message: {:?}", why);
    }
}

async fn send_pickers(ctx: &Context, msg: &Message, pickers: &[cobalt::PickerItem]) {
    use futures::future::join_all;

    let futures = pickers.iter().map(mapper).collect::<Vec<_>>();
    let results = join_all(futures).await;
    let results = results
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let builder = CreateMessage::default()
        .add_files(results.into_iter())
        .reference_message(MessageReference::from(msg));

    if let Err(e) = msg.channel_id.send_message(ctx, builder).await {
        eprintln!("Error sending message {:?}", e);
    }
}

async fn mapper(item: &cobalt::PickerItem) -> anyhow::Result<CreateAttachment> {
    let response = reqwest::get(&item.url).await?;
    let bytes = response.bytes().await?;
    Ok(CreateAttachment::bytes(bytes, "thumb.png".to_string()))
}
