use eyre::{Result, Report};
use serenity::prelude::*;
use serenity::builder::CreateApplicationCommand;
use serenity::{
  model::{
    application::interaction::application_command::{ApplicationCommandInteraction},
  },
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("register").description("Register guild")
}

/// This will be called by the admin of the Guild. It will basically load the information we need.
/// This includes all the channels for the given guild as well as all roles
pub async fn run(ctx: &Context, cmd: &mut ApplicationCommandInteraction) -> Result<String> {
  let member = cmd.member.take();
  let is_admin = member
  .ok_or(Report::msg("error"))?
  .permissions.ok_or(Report::msg("error"))?
  .administrator();

  if !is_admin {
    return Err(Report::msg("error"))
  }

  // TODO: store guild_channels and guild_roles in the DB
  let _guild_channels = ctx.cache.guild_channels(cmd.guild_id.unwrap()).unwrap();
  let _guild_roles = ctx.cache.guild_roles(cmd.guild_id.unwrap()).unwrap();

  println!("{:?}", _guild_roles);

  Ok("Record updated".to_string())
}
