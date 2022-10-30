use std::env;

pub struct Config {
  pub discord_token: String,
  pub discord_client_id: String,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Result::Ok(
      Self {
        discord_token: env::var("DISCORD_TOKEN").unwrap(),
        discord_client_id: env::var("DISCORD_CLIENT_ID").unwrap(),
      }
    )
  }
}
