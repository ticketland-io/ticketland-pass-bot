use std::time::{UNIX_EPOCH, SystemTime};
use eyre::Result;
use ticketland_crypto::{
  symetric::hmac,
};

pub fn create_access_token(api_client_id: String, api_client_key: String) -> Result<String> {
  let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
  let msg = format!("{}:{}", api_client_id, ts);
  
  hmac::sign_sha256(&api_client_key, &msg)
}
