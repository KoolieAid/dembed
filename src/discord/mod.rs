use serenity::prelude::*;
use std::env;

pub mod bot;

pub async fn make() {
    let token = env::var("DISCORD_TOKEN").expect("No token found");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let handler = bot::Handler {
        cobalt_client: reqwest::Client::new(),
    };

    let mut discord_client = Client::builder(&token, intents)
        .event_handler(handler)
        .intents(intents)
        .await
        .expect("Error creating client");

    if let Err(why) = discord_client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
