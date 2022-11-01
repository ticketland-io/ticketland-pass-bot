use std::env;

pub struct Config {
  pub discord_token: String,
  pub discord_client_id: String,
  pub api_client_id: String,
  pub api_client_key: String,
  pub neo4j_host: String,
  pub neo4j_domain: Option<String>,
  pub neo4j_username: String,
  pub neo4j_password: String,
  pub neo4j_database: Option<String>,
  pub ticketland_pass_dapp: String,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Result::Ok(
      Self {
        discord_token: env::var("DISCORD_TOKEN").unwrap(),
        discord_client_id: env::var("DISCORD_CLIENT_ID").unwrap(),
        api_client_id: env::var("API_CLIENT_ID").unwrap(),
        api_client_key: env::var("API_CLIENT_KEY").unwrap(),
        neo4j_host: env::var("NEO4J_HOST").unwrap(),
        neo4j_domain: None,
        neo4j_username: env::var("NEO4J_USERNAME").unwrap(),
        neo4j_password: env::var("NEO4J_PASSWORD").unwrap(),
        neo4j_database: env::var("NEO4J_DATABASE").ok(),
        ticketland_pass_dapp: env::var("TICKETLAND_PASS_DAPP").unwrap(),
      }
    )
  }
}
