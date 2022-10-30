use std::{
  env,
  sync::Arc,
};
use tokio::sync::Mutex;
use serenity::prelude::*;
use ticketland_pass_bot::{
  utils::store::Store,
  handler::Handler,
  role_manager,
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

  let client = Client::builder(store.config.discord_token.clone(), intents)
    .event_handler(Handler)
    .await
    .expect("Error creating client");
  let client = Arc::new(Mutex::new(client));
  
  let client_clone = Client::builder(store.config.discord_token, intents)
  .event_handler(Handler)
  .await
  .expect("Error creating client");

  tokio::spawn(async move {
    role_manager::start(client_clone).await;
  });
  
  // start listening for events by starting a single shard
  let mut client = client.lock().await;
  if let Err(error) = client.start().await {
    println!("An error occurred while running the client: {:?}", error);
  }
}
