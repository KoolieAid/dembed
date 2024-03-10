use serenity::all::CreateAttachment;
use serenity::prelude::Context;

#[derive(Debug)]
pub enum MessageType {
    // For free users
    URL(String),
    // For premium users
    Attachment(CreateAttachment),
}

pub trait Sendable {
    async fn send_single(&self, ctx: &Context, msg_content: MessageType) -> anyhow::Result<()>;

    async fn send_multiple(&self, ctx: &Context, msg_content: Vec<MessageType>) -> anyhow::Result<()>;
}

