use std::env;

pub struct SpApiConfig {
  pub app_id: String,
  pub is_draft: bool,
}

impl SpApiConfig {
  pub fn from_env() -> Self {
    Self { 
      app_id: env::var("AMAZON_SP_API_APP_ID").expect("env AMAZON_SP_API_APP_ID"),
      is_draft: env::var("AMAZON_SP_API_IS_DRAFT").ok().is_some(),
    }
  }
}