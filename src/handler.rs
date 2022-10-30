use tracing::{error, info};
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::{
  model::{
    application::interaction::{Interaction, InteractionResponseType},
    event::ResumedEvent,
    gateway::Ready,
  },
};
use crate::cmd::{
  verify_ticket,
  register_server,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
      let content = match command.data.name.as_str() {
        "verify" => verify_ticket::run(&ctx, &command).await,
        "register" => register_server::run(&ctx, &command).await,
        _ => "not implemented :(".to_string(),
      };

      if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
          response
          .kind(InteractionResponseType::ChannelMessageWithSource)
          .interaction_response_data(|message| message.content(content))
        })
        .await
      {
        error!("Cannot respond to slash command: {}", why);
      }
    }

    let _ = Command::create_global_application_command(&ctx.http, |command| {
      verify_ticket::register(command);
      register_server::register(command)
    })
    .await;
  }

  async fn ready(&self, _: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);
  }

  async fn resume(&self, _: Context, _: ResumedEvent) {
    info!("Resumed");
  }
}
