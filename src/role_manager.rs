use eyre::Result;
use serenity::prelude::*;

pub async fn start(client: Client) -> Result<()> {
  // TODO: simulate adding a role after ticket is verifier
  tokio::time::sleep(std::time::Duration::from_secs(5)).await;

  // We get this data from RabbitMQ message
  let guild_id = 998970106598129754;
  let user_id = 113267696933535744;
  let role_name = "ticketland-pass";

  let role = client.cache_and_http.http
  .get_guild_roles(guild_id)
  .await?
  .into_iter()
  .find(|r| r.name == role_name)
  .unwrap();

  let result = client.cache_and_http.http
  .get_member(guild_id, user_id)
  .await?
  .add_role(&client.cache_and_http.http, role.id)
  .await;

  println!("Result is {:?}", result);

  Ok(())
}
