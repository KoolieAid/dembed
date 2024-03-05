use serenity::prelude::*;
use serenity::model::channel::Message;

pub struct Handler {
    pub cobalt_client: reqwest::Client,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, msg: Message) {
        println!("Message: {:?}", msg.content);
    }
}
