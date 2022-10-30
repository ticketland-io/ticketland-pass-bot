use eyre::{Result, Report};
use serenity::prelude::*;
use serenity::builder::CreateApplicationCommand;
use serenity::{
  model::{
    application::interaction::application_command::{ApplicationCommandInteraction},
  },
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("verify").description("Verify pass")
}

pub async fn run(_: &Context, cmd: &ApplicationCommandInteraction) -> Result<String> {
  // store this in Redis
  let _guild_id = cmd.guild_id.ok_or(Report::msg("error"))?;
  let _user_id = cmd.user.id;

  // TODO: load the event id associated with the Guild from which this channel was invoked
  Ok("Verify your pass https://apps.ticketland.io/discord".to_string())
}
