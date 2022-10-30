use serenity::prelude::*;
use serenity::builder::CreateApplicationCommand;
use serenity::{
  model::{
    application::interaction::application_command::{ApplicationCommandInteraction},
  },
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("register").description("A ping command")
}

/// This will be called by the 
pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> String {
  let guild_channels = ctx.cache.guild_channels(cmd.guild_id.unwrap()).unwrap();
  format!("{:?}", guild_channels)
}
