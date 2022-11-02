use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct RoleAssignment {
  pub discord_uid: String,
  pub guild_id: String,
  pub roles: Vec<String>,
}
