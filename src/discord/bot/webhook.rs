use serenity::all::ExecuteWebhook;
use serenity::all::Member;
use serenity::builder::CreateWebhook;
use serenity::model::channel::GuildChannel;
use serenity::model::webhook::Webhook;
use serenity::prelude::*;

use crate::cobalt;

async fn get_any_webhook(ctx: &Context, channel: &GuildChannel) -> anyhow::Result<Webhook> {
    use serenity::model::webhook::WebhookType;
    let webhooks = channel
        .webhooks(ctx)
        .await?
        .into_iter()
        .filter(|w| w.kind == WebhookType::Incoming)
        .filter(|w| w.token.is_some())
        .collect::<Vec<_>>();

    if !webhooks.is_empty() {
        return Ok(webhooks.into_iter().next().unwrap());
    }

    let wb_builder = CreateWebhook::new("Dembed").audit_log_reason("Creating webhook for Dembed");

    let webhook = channel.create_webhook(ctx, wb_builder).await?;
    Ok(webhook)
}

pub async fn send_link(ctx: &Context, chn: &GuildChannel, member: &Member, pickers: &[cobalt::PickerItem]) -> anyhow::Result<()> {
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

    let webhook = get_any_webhook(ctx, chn).await?;

    let builder = ExecuteWebhook::default()
        .add_files(results.into_iter())
        .username(member.display_name())
        .avatar_url(member.face());

    webhook.execute(ctx, false, builder).await?;
    Ok(())
}
