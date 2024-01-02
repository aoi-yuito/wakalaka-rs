use crate::util::uses::*;

mod booru;
mod core;
mod util;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

#[tokio::main]
pub async fn main() {
    let options = framework::setup_framework_options().await;

    let framework = framework::build_framework(options).await;

    let settings = match Settings::new() {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };
    let intents =
        GatewayIntents::default() | GatewayIntents::GUILD_MEMBERS | GatewayIntents::MESSAGE_CONTENT;

    let client = ClientBuilder::new(settings.discord_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
