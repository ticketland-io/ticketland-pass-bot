use std::sync::Arc;
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
use crate::{
  utils::store::Store,
  cmd::{
    verify_ticket::VerifyCmd,
    register::RegisterCmd,
  },  
};

pub struct Handler {
  register_cmd: RegisterCmd,
  verify_cmd: VerifyCmd,
}

impl Handler {
  pub fn new(store: Arc<Store>) -> Self {
    Self {
      register_cmd: RegisterCmd::new(Arc::clone(&store)),
      verify_cmd: VerifyCmd::new(Arc::clone(&store)),
    }
  }
}

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(mut command) = interaction {
      let content = match command.data.name.as_str() {
        "register" => self.register_cmd.run(&ctx, &mut command).await.unwrap_or("Error".to_string()),
        "verify" => self.verify_cmd.run(&ctx, &command).await.unwrap_or("Error".to_string()),
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
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);

    let _ = Command::create_global_application_command(&ctx.http, |command| {
      RegisterCmd::register(command);
      VerifyCmd::register(command)
    })
    .await;
  }

  async fn resume(&self, _: Context, _: ResumedEvent) {
    info!("Resumed");
  }
}
