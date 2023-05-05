use std::{
  env,
  sync::Arc,
};
use tracing::error;
use tokio::sync::Mutex;
use serenity::prelude::*;
use amqp_helpers::consumer::consumer_runner::ConsumerRunner;
use ticketland_pass_bot::{
  utils::store::Store,
  handler::Handler,
  queue_consumer::role_handler::RoleHandler,
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

  let store = Arc::new(Store::new().await);

  let intents = GatewayIntents::GUILDS
  | GatewayIntents::GUILD_MESSAGES
  | GatewayIntents::MESSAGE_CONTENT;

  let handler = Handler::new(Arc::clone(&store));
  let client = Client::builder(store.config.discord_token.clone(), intents)
    .event_handler(handler)
    .await
    .expect("Error creating client");
  let client = Arc::new(Mutex::new(client));

  tokio::spawn(async move {
    let client = Client::builder(store.config.discord_token.clone(), intents)
      .await
      .expect("Error creating client");
    
    let role_handler_consumer = ConsumerRunner::new(
      store.config.rabbitmq_uri.clone(),
      "discord_roles".to_owned(),
      "discord_roles".to_owned(),
      1,
      RoleHandler::new(client).await,
    ).await.unwrap();

    role_handler_consumer.start().await.unwrap();
  });

  // start listening for events by starting a single shard
  let mut client = client.lock().await;
  if let Err(error) = client.start().await {
    error!("An error occurred while running the client: {:?}", error);
  }
}
