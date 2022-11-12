use std::sync::Arc;
use eyre::{Result, Report};
use tracing::info;
use serenity::prelude::*;
use async_trait::async_trait;
use lapin::{
  message::{Delivery},
};
use amqp_helpers::core::types::Handler;
use crate::{
  models::role_assignment::RoleAssignment,
};

pub struct RoleHandler {
  client: Client,
}

impl RoleHandler {
  pub async fn new(client: Client) -> Self {
    Self {
      client,
    }
  }
}

#[async_trait]
impl Handler<RoleAssignment> for RoleHandler {
  async fn handle(&self, msg: RoleAssignment, _: &Delivery) -> Result<()> {
    info!("Assigning roles to {} in guild {}", &msg.discord_uid, &msg.guild_id);

    let guild_id = msg.guild_id.parse::<u64>()?;
    let discord_uid = msg.discord_uid.parse::<u64>()?;
    
    let roles: Vec<u64> = msg.roles.into_iter()
    .map(|r| r.parse::<u64>().map_err(|_| Report::msg("Invalid role id")))
    .filter(|r| r.is_ok())
    .map(|r| r.unwrap())
    .collect();

    let member = self.client.cache_and_http.http.get_member(guild_id, discord_uid).await?;
    for role_id in roles {
      member
      .clone()
      .add_role(Arc::clone(&self.client.cache_and_http.http), role_id)
      .await?;
    }

    info!("Roles Assigned to {} in guild {}", &msg.discord_uid, &msg.guild_id);
    Ok(())
  }
}
