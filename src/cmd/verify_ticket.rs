use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command};
use serenity::framework::standard::{CommandResult};

#[command]
pub async fn verify(ctx: &Context, msg: &Message) -> CommandResult {
  msg.reply(ctx, "Pong!").await?;

  Ok(())
}
