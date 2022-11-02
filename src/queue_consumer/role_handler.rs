use std::sync::Arc;
use eyre::Result;
use async_trait::async_trait;
use lapin::{
  message::{Delivery},
};
use amqp_helpers::core::types::Handler;
use crate::{
  utils::store::Store,
  models::role_assignment::RoleAssignment,
};

pub struct RoleHandler {
  _store: Arc<Store>,
}

impl RoleHandler {
  pub async fn new(_store: Arc<Store>) -> Self {
    Self {
      _store,
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
