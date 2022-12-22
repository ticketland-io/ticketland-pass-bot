use ticketland_pass_common_data::connection_pool::ConnectionPool;
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub pg_pool: ConnectionPool,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();
    let pg_pool = ConnectionPool::new(&config.postgres_uri).await;

    Self {
      config,
      pg_pool,
    }
  }
}
