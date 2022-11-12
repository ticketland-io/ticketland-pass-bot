use std::sync::Arc;
use tokio::sync::Mutex;
use ticketland_pass_common_data::connection::PostgresConnection;
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub postgres: Arc<Mutex<PostgresConnection>>,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();
    let postgres = Arc::new(Mutex::new(PostgresConnection::new(&config.postgres_uri).await));

    Self {
      config,
      postgres,
    }
  }
}
