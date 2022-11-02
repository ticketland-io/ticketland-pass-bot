use eyre::Result;
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
    println!("Assigning roles to {} in guild {}", &msg.discord_uid, &msg.guild_id);

    Ok(())
  }
}
