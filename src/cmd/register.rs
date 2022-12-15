use std::sync::Arc;
use eyre::{Result, Report};
use serenity::prelude::*;
use serenity::builder::CreateApplicationCommand;
use serenity::{
  model::{
    application::interaction::application_command::{ApplicationCommandInteraction},
  },
};
use ticketland_pass_common_data::{
  models::{
    account::Account,
    guild_admin::GuildAdmin,
    guild::Guild,
    reg_session::NewRegSession,
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

  async fn is_registered(&self, discord_uid: String, guild_id: String) -> Result<()> {
    let mut postgres = self.store.postgres.lock().await;
    postgres.read_guild_admins(discord_uid, guild_id).await?; 
    
    return Ok(())
  }

  async fn create_new_account(&self, discord_uid: String) -> Result<String> {
    let session_id = Id::new();
    let mut postgres = self.store.postgres.lock().await;

    let account = Account {
      discord_uid: discord_uid.clone(),
      ..Default::default()
    };
    let reg_session = NewRegSession {
      account_id: discord_uid.clone(),
      session_id: session_id.clone(),
    };

    postgres.create_account(account, reg_session).await?;
    
    Ok(session_id.to_string())
  }
  
  async fn add_guild(&self, discord_uid: String, guild: Guild) -> Result<()> {
    let mut postgres = self.store.postgres.lock().await;
    let guild_admin = GuildAdmin {
      guild_id: guild.guild_id.clone(),
      account_id: discord_uid,
      ..Default::default()
    };

    Ok(postgres.add_user_guild(guild, guild_admin).await?)
  }

  /// This will be called by the admin of the Guild. It will basically load the information we need.
  /// This includes all the channels for the given guild as well as all roles
  pub async fn run(&self, ctx: &Context, cmd: &mut ApplicationCommandInteraction) -> Result<String> {
    let member = cmd.member.take().ok_or(Report::msg("error"))?;
    let discord_uid = member.user.id.to_string();
    let is_admin = member
    .permissions.ok_or(Report::msg("error"))?
    .administrator();

    if !is_admin {
      return Err(Report::msg("error"))
    }

    let guild_id = cmd.guild_id.ok_or(Report::msg("error"))?;
    if self.is_registered(discord_uid.clone(), guild_id.to_string()).await.is_ok() {
      return Ok("You have already registered this Server".to_string())
    }
    
    let session_id = self.create_new_account(discord_uid.clone()).await?;

    // save guild
    let guild = ctx.http.get_guild(guild_id.0).await?;
    let guild = Guild {
      guild_id: guild_id.to_string(),
      name: guild.name,
      icon: guild.icon,
      ..Default::default()
    };

    self.add_guild(discord_uid, guild).await?;

    Ok(format!(
      "{}/register?&session_id={}&guild_id={}&channel_id={}",
      self.store.config.ticketland_pass_dapp,
      session_id,
      guild_id.to_string(),
      cmd.channel_id.to_string(),
    ))
  }
}
