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
  repositories::account::create_account,
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

  async fn create_new_account(&self, discord_uid: String) -> Result<String> {
    let session_id = Id::new();
    let (query, db_query_params) = create_account(discord_uid, session_id.to_string());
    
    // send_write(
    //   Arc::clone(&self.store.neo4j),
    //   query,
    //   db_query_params,
    // ).await

    todo!()
  }
  

  /// This will be called by the admin of the Guild. It will basically load the information we need.
  /// This includes all the channels for the given guild as well as all roles
  pub async fn run(&self, ctx: &Context, cmd: &mut ApplicationCommandInteraction) -> Result<String> {
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
}
