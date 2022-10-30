use serenity::prelude::*;

pub async fn start(client: Client) {
  // TODO: simulate adding a role after ticket is verifier
  let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
  interval.tick().await;

  let cache = std::sync::Arc::clone(&client.cache_and_http.cache);
  // We can load this dynamically from the cache
  let role_id = 998970106598129754;
  let result = cache.member(998970106598129754, 113267696933535744)
  .unwrap()
  .add_role(&client.cache_and_http.http, role_id)
  .await;

  println!("Result is {:?}", result);  
}
