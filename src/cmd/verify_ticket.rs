use std::sync::Arc;
use eyre::{Result, Report};
use serenity::prelude::*;
use serenity::builder::CreateApplicationCommand;
use serenity::{
  model::{
    application::interaction::application_command::{ApplicationCommandInteraction},
  },
};
use crate::utils::store::Store;

pub struct VerifyCmd {
  store: Arc<Store>,
}

impl VerifyCmd {
  pub fn new(store: Arc<Store>) -> Self {
    Self {store}
  }

  pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("verify").description("Verify pass")
  }

  pub async fn run(&self, _: &Context, cmd: &ApplicationCommandInteraction) -> Result<String> {
    let guild_id = cmd.guild_id.ok_or(Report::msg("error"))?;
    let discord_uid = cmd.user.id;

    Ok(format!(
      "{}/verify?&discord_uid={}&guild_id={}&channel_id={}",
      self.store.config.ticketland_pass_dapp,
      discord_uid,
      guild_id.to_string(),
      cmd.channel_id.to_string(),
    ))
  }
  
}
