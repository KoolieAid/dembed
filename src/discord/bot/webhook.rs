use serenity::all::ExecuteWebhook;
use serenity::builder::CreateWebhook;
use serenity::model::channel::GuildChannel;
use serenity::model::channel::Message;
use serenity::model::webhook::Webhook;
use serenity::prelude::*;

use crate::cobalt;

use super::types::MessageType;
use super::types::Sendable;

impl Sendable for Webhook {
    async fn send_single(&self, ctx: &Context, msg_content: MessageType) -> anyhow::Result<()> {
        match msg_content {
            MessageType::URL(content) => {}
            MessageType::Attachment(attachment) => {}
        }

        Ok(())
    }

    async fn send_multiple(&self, ctx: &Context, msg_content: Vec<MessageType>) -> anyhow::Result<()> {
        todo!()
    }
}

pub async fn get_any_webhook(ctx: &Context, channel: &GuildChannel) -> anyhow::Result<Webhook> {
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

async fn send_single(ctx: &Context, msg: &Message, url: &str) -> anyhow::Result<()> {
    use serenity::model::channel::Channel;
    let channel = msg.channel(ctx).await?;
    let webhook = match channel {
        Channel::Guild(channel) => get_any_webhook(ctx, &channel).await?,
        Channel::Private(_channel) => todo!("Handle private channel"),
        _ => todo!("Handle new type of channel"),
    };

    let builder = ExecuteWebhook::default()
        .add_file(super::convert_to_attachment(url).await?)
        .username(msg.member(ctx).await?.display_name())
        .avatar_url(msg.author.avatar_url().unwrap_or_default());

    webhook.execute(ctx, false, builder).await?;
    msg.delete(ctx).await?;
    Ok(())
}

async fn send_multiple(ctx: &Context, msg: &Message, pickers: &[cobalt::PickerItem]) -> anyhow::Result<()> {
    use futures::future::join_all;

    let futures = pickers
        .iter()
        .map(|item| super::convert_to_attachment(&item.url))
        .collect::<Vec<_>>();
    let results = join_all(futures).await;
    let results = results
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    use serenity::model::channel::Channel;
    let channel = msg.channel(ctx).await?;
    let webhook = match channel {
        Channel::Guild(channel) => get_any_webhook(ctx, &channel).await?,
        Channel::Private(_channel) => todo!("Handle private channel"),
        _ => todo!("Handle new type of channel"),
    };

    let builder = ExecuteWebhook::default()
        .add_files(results.into_iter())
        .username(msg.member(ctx).await?.display_name())
        .avatar_url(msg.author.avatar_url().unwrap_or_default());

    webhook.execute(ctx, false, builder).await?;
    msg.delete(ctx).await?;
    Ok(())
}
