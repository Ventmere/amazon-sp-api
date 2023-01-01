use std::ops::Deref;

use amazon_sp_api_shared::configuration::AWSv4Key;

use crate::{
    shared::{
        configuration::Configuration,
        auth::{AuthState, AuthScope}
    },
    config::SpApiConfig, SpApiEndpoint,
};

#[derive(Debug)]
pub struct SpApiClient {
    config: Configuration,
}

impl SpApiClient {
    pub fn new(config: SpApiConfig, refresh_token: String, endpoint: SpApiEndpoint) -> Self {
        let config = Configuration {
            client: reqwest::Client::new(),
            base_path: endpoint.get_url().to_string(),
            aws_v4_key: AWSv4Key {
                access_key: config.aws_access_key_id,
                secret_key: config.aws_secret_access_key.into(),
                region: endpoint.get_aws_region().to_string(),
                service: "execute-api".to_string(),
            }.into(),
            auth: Some(AuthState::oauth_refresh_token(
                config.client_id, 
                config.client_secret, 
                refresh_token)),
            ..Default::default()
        };
        Self {
            config,
        }
    }
}

impl Deref for SpApiClient {
    type Target = Configuration;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}