use std::sync::Arc;
use eyre::{Result, Report};
use serenity::prelude::*;
use serenity::builder::CreateApplicationCommand;
use serenity::{
  model::{
    application::interaction::application_command::{ApplicationCommandInteraction},
  },
};
use common_data::{
  helpers::{send_read, send_write},
};
use ticketland_pass_common_data::{
  models::guild::Guild,
  repositories::{
    guild::{add_user_guild, get_user_guild},
    account::create_account,
  },
};
use ticketland_crypto::utils::id::Id;
use crate::utils::store::Store;

pub struct RegisterCmd {
  store: Arc<Store>,
}

impl RegisterCmd {
  pub fn new(store: Arc<Store>) -> Self {
    Self {store}
  }

  pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("register").description("Register guild")
  }

  async fn is_registered(&self, discord_uid: String, guild_id: String) -> Result<Guild> {
    let (query, db_query_params) = get_user_guild(discord_uid, guild_id);

    let guild = send_read(
      Arc::clone(&self.store.neo4j),
      query,
      db_query_params,
    ).await
    .map(TryInto::<Guild>::try_into)??;

    Ok(guild)
  }

  async fn create_new_account(&self, discord_uid: String) -> Result<String> {
    let session_id = Id::new();
    let (query, db_query_params) = create_account(discord_uid, session_id.to_string());
    
    send_write(
      Arc::clone(&self.store.neo4j),
      query,
      db_query_params,
    ).await?;

    Ok(session_id.to_string())
  }
  
  async fn add_guild(&self, discord_uid: String, guild: Guild) -> Result<()> {
    let (query, db_query_params) = add_user_guild(discord_uid, guild);
    
    send_write(
      Arc::clone(&self.store.neo4j),
      query,
      db_query_params,
    ).await?;

    Ok(())
  }

  /// This will be called by the admin of the Guild. It will basically load the information we need.
  /// This includes all the channels for the given guild as well as all roles
  pub async fn run(&self, ctx: &Context, cmd: &mut ApplicationCommandInteraction) -> Result<String> {
    let member = cmd.member.take().ok_or(Report::msg("error"))?;
    let is_admin = member
    .permissions.ok_or(Report::msg("error"))?
    .administrator();

    if !is_admin {
      return Err(Report::msg("error"))
    }

    let guild_id = cmd.guild_id.ok_or(Report::msg("error"))?;
    if self.is_registered(member.user.id.to_string(), guild_id.to_string()).await.is_err() {
      return Ok("You have already registered this Server".to_string())
    }
    
    let session_id = self.create_new_account(member.user.id.to_string()).await?;

    // save guild
    let guild = ctx.http.get_guild(guild_id.0).await?;
    let guild = Guild {
      id: guild_id.to_string(),
      name: guild.name,
      icon: guild.icon,
    };

    self.add_guild(session_id.clone(), guild).await?;

    Ok(format!(
      "{}/register?&session_id={}guild_id={}&channel_id={}",
      self.store.config.ticketland_pass_dapp,
      session_id,
      guild_id.to_string(),
      cmd.channel_id.to_string(),
    ))
  }
}
