use serenity::prelude::*;
use serenity::framework::standard::{StandardFramework};
use ticketland_pass_bot::{
  utils::store::Store,
  cmd::{GENERAL_GROUP, Handler},
};

#[tokio::main]
async fn main() {
  let store = Store::new().await;

  let framework = StandardFramework::new()
  .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
  .group(&GENERAL_GROUP);

  let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
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
