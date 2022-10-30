use serenity::prelude::*;
use super::config::Config;

pub struct Store {
  pub config: Config,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();


    Self {
      config,
    }
  }
}
