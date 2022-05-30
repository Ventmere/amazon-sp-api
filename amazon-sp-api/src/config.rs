use std::env;

pub struct SpApiConfig {
  pub app_id: String,
}

impl SpApiConfig {
  pub fn from_env() -> Self {
    Self { 
      app_id: env::var("AMAZON_SP_API_APP_ID").expect("env AMAZON_SP_API_APP_ID")
    }
  }
}