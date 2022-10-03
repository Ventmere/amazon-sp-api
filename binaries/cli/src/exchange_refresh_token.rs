use clap::Parser;
use amazon_sp_api::{
  shared::auth::{AuthState},
  config::SpApiConfig,
};
use anyhow::Result;

#[derive(Debug, Parser)]
pub struct AuthArgs {
  endpoint: amazon_sp_api::SpApiEndpoint,
}

pub async fn run(code: String) -> Result<()> {
  let client = reqwest::Client::new();
  let sp_api_config = SpApiConfig::from_env();
  let auth_state = AuthState::oauth_code(
    sp_api_config.client_id, 
    sp_api_config.client_secret,
    code
  );

  auth_state.get_access_token::<()>(&client).await?;

  Ok(())
}