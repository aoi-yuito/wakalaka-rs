use crate::util::uses::*;

mod booru;
mod core;
mod util;

// const BOT_INVITE_URL: &str = "https://discord.com/api/oauth2/authorize?client_id=1190718691055251548&permissions=8&scope=bot";

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

async fn setup_framework_options() -> poise::FrameworkOptions<Data, Error> {
    let options = poise::FrameworkOptions {
        commands: vec![crate::booru::aibooru::aibooru()],
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command '{}'...", ctx.command().qualified_name);
            })
        },
        post_command: |_| Box::pin(async move {}),
        event_handler: |ctx, event, framework, data| {
            Box::pin(crate::event::event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };
    options
}

async fn setup_framework<'a>(
    ctx: &serenity::Context,
    _ready: &Ready,
    framework: &poise::Framework<Data, Error>,
) -> Result<Data, Error> {
    poise::builtins::register_globally(ctx, &framework.options().commands).await?;

    Ok(Data {})
}

#[tokio::main]
pub async fn main() {
    let options = setup_framework_options().await;

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, ready, framework| {
            Box::pin(async move { setup_framework(ctx, ready, framework).await })
        })
        .build();

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
