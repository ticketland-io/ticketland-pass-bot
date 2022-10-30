use std::env;
use serenity::prelude::*;
use ticketland_pass_bot::{
  utils::store::Store,
  handler::Handler,
};

#[tokio::main]
async fn main() {
  if env::var("ENV").unwrap() == "development" {
    dotenv::from_filename(".env").expect("cannot load env from a file");
  }

  // Initialize the logger to use environment variables.
  // In this case, a good default is setting the environment variable
  // `RUST_LOG` to `debug`.
  tracing_subscriber::fmt::init();

  let store = Store::new().await;

  let intents = GatewayIntents::GUILDS
  | GatewayIntents::GUILD_MESSAGES
  | GatewayIntents::MESSAGE_CONTENT;

  let mut client = Client::builder(store.config.discord_token, intents)
  .event_handler(Handler)
  .await
  .expect("Error creating client");

  // start listening for events by starting a single shard
  if let Err(error) = client.start().await {
    println!("An error occurred while running the client: {:?}", error);
  }
}
