use std::env;

#[derive(Debug, Clone)]
pub struct SpApiConfig {
  pub app_id: String,
  pub is_draft: bool,
  pub client_id: String,
  pub client_secret: String,
  pub aws_access_key_id: String,
  pub aws_secret_access_key: String,
}

impl SpApiConfig {
  pub fn from_env() -> Self {
    Self { 
      app_id: env::var("AMAZON_SP_API_APP_ID").expect("env AMAZON_SP_API_APP_ID"),
      is_draft: env::var("AMAZON_SP_API_IS_DRAFT").ok().is_some(),
      client_id: env::var("AMAZON_SP_API_CLIENT_ID").expect("env AMAZON_SP_API_APP_ID"),
      client_secret: env::var("AMAZON_SP_API_CLIENT_SECRET").expect("env AMAZON_SP_API_CLIENT_SECRET"),
      aws_access_key_id: env::var("AWS_ACCESS_KEY_ID").expect("env AWS_ACCESS_KEY_ID"),
      aws_secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").expect("env AWS_SECRET_ACCESS_KEY"),
    }
  }
}