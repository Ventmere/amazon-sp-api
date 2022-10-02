use clap::Parser;
use amazon_sp_api::{
  shared::auth::{AuthState, AuthScope},
  config::SpApiConfig,
};
use anyhow::Result;
use amazon_sp_authorization::apis::configuration::{
  Configuration,
  AWSv4Key
};

#[derive(Debug, Parser)]
pub struct AuthArgs {
  endpoint: amazon_sp_api::SpApiEndpoint,
}

pub async fn run() -> Result<()> {
  let client = reqwest::Client::new();
  let sp_api_config = SpApiConfig::from_env();
  let region = amazon_sp_api::SpApiEndpoint::Europe;
  let config = Configuration {
    client,
    base_path: region.get_url().to_string(),
    aws_v4_key: AWSv4Key {
      access_key: sp_api_config.aws_access_key_id,
      secret_key: sp_api_config.aws_secret_access_key.into(),
      region: region.get_aws_region().to_string(),
      service: "execute-api".to_string(),
    }.into(),
    auth: Some(AuthState::grantless(
      sp_api_config.client_id, 
      sp_api_config.client_secret, 
      AuthScope::Migration)),
    ..Default::default()
  };
  amazon_sp_authorization::apis::authorization_api::get_authorization_code(
    &config, 
    "A2YBGR89PRW3VX", 
    "054564885717",
    "amzn.mws.58b832cc-ef62-c2c4-fe5d-f0741666bca8"
  ).await?;

  Ok(())
}