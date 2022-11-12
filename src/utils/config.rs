use std::env;

pub struct Config {
  pub postgres_uri: String,
  pub discord_token: String,
  pub discord_client_id: String,
  pub api_client_id: String,
  pub api_client_key: String,
  pub signer_keypair: String,
  pub rabbitmq_uri: String,
  pub ticketland_pass_dapp: String,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Result::Ok(
      Self {
        postgres_uri: env::var("POSTGRES_URI").unwrap(),
        discord_token: env::var("DISCORD_TOKEN").unwrap(),
        discord_client_id: env::var("DISCORD_CLIENT_ID").unwrap(),
        api_client_id: env::var("API_CLIENT_ID").unwrap(),
        api_client_key: env::var("API_CLIENT_KEY").unwrap(),
        signer_keypair: env::var("SIGNER_KEYPAIR").unwrap(),
        rabbitmq_uri: env::var("RABBITMQ_URI").unwrap(),
        ticketland_pass_dapp: env::var("TICKETLAND_PASS_DAPP").unwrap(),
      }
    )
  }
}
