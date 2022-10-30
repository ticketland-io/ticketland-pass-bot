use serenity::prelude::*;
use serenity::framework::standard::{StandardFramework};
use ticketland_pass_bot::{
  utils::store::Store,
  cmd::{GENERAL_GROUP},
  handler::Handler,
};

#[tokio::main]
async fn main() {
  // Initialize the logger to use environment variables.
  // In this case, a good default is setting the environment variable
  // `RUST_LOG` to `debug`.
  tracing_subscriber::fmt::init();

  let store = Store::new().await;

  let framework = StandardFramework::new()
  .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
  .group(&GENERAL_GROUP);

  let intents = GatewayIntents::GUILDS
  | GatewayIntents::GUILD_MESSAGES
  | GatewayIntents::MESSAGE_CONTENT;

  let mut client = Client::builder(store.config.discord_token, intents)
  .event_handler(Handler)
  .framework(framework)
  .await
  .expect("Error creating client");

  // start listening for events by starting a single shard
  if let Err(error) = client.start().await {
    println!("An error occurred while running the client: {:?}", error);
  }
}
