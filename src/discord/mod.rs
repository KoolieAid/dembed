use dotenv_codegen::dotenv;
use serenity::prelude::*;

pub mod bot;

pub async fn make() {
    let token = dotenv!("DISCORD_TOKEN");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let handler = bot::Handler {
        cobalt_client: cobalt::Cobalt::default(),
    };

    let mut discord_client = Client::builder(token, intents)
        .event_handler(handler)
        .intents(intents)
        .await
        .expect("Error creating client");

    if let Err(why) = discord_client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
