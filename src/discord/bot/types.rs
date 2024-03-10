use serenity::all::{CreateAttachment, Webhook};
use serenity::prelude::Context;

#[derive(Debug)]
pub enum MessageType {
    // For free users
    URL(String),
    // For premium users
    Attachment(CreateAttachment),
}

pub trait Sendable {
    async fn send(&self, ctx: &Context, msg_content: MessageType) -> anyhow::Result<()>;
}

impl Sendable for Webhook {
    async fn send(&self, ctx: &Context, msg_content: MessageType) -> anyhow::Result<()> {
        match msg_content {
            MessageType::URL(content) => {
            },
            MessageType::Attachment(attachment) => {
            },
        }

        Ok(())
    }
}

