use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{Instant, Duration};
use reqwest::Client;
use serde_derive::{Serialize, Deserialize};
use crate::error::{SharedError};
use crate::request::ResponseContent;

#[derive(Debug, Clone)]
pub struct AuthState {
  client_id: String,
  client_secret: String,
  auth_type: AuthType,
  token: Arc<RwLock<Option<BearerToken>>>,
}

impl AuthState {
  pub fn grantless(client_id: String, client_secret: String, scope: AuthScope) -> Self {
    Self {
      client_id,
      client_secret,
      auth_type: AuthType::Grantless { 
        scope: scope.as_str() 
      },
      token: Arc::new(RwLock::new(None)),
    }
  }

  pub fn reset_access_token(&self) {
    self.token.write().take();
  }

  pub async fn get_access_token<T>(&self, client: &Client) -> Result<String, SharedError<T>> {
    if let Some(token) = self.token.read().as_ref().map(|v| v.clone()) {
      if token.expires_at.checked_duration_since(Instant::now()).map(|d| d > Duration::from_secs(60)).unwrap_or_default() {
        tracing::debug!("reuse token");
        return Ok(token.token)
      }
      self.token.write().take();
    }

    match &self.auth_type {
      AuthType::Grantless { scope } => {
        tracing::debug!("getting token using client_credentials");
        let req = GrantRequest {
          grant_type: "client_credentials",
          scope,
          client_id: &self.client_id,
          client_secret: &self.client_secret,
        };
        let res = client.post("https://api.amazon.com/auth/o2/token")
          .json(&req)
          .send().await?;
        let status = res.status();
        let body = res.text().await.ok();
        if !status.is_success() {
          return Err(
            SharedError::ResponseError(ResponseContent {
                status,
                content: body.unwrap_or_default(),
                entity: None,
            })
          )
        }
  
        let reply: ClientCredentialsGrantReply = if let Some(ref body ) = body {
          serde_json::from_str(body)?
        } else {
          return Err(
            SharedError::ResponseError(ResponseContent {
              status,
              content: "".to_string(),
              entity: None,
            })
          )
        };
  
        let token = reply.access_token;
        let expires_at = Instant::now() + Duration::from_secs(reply.expires_in as _);

        tracing::debug!("token = {}, expires_in = {}", token, reply.expires_in);

        self.token.write().replace(BearerToken {
          token: token.clone(),
          expires_at
        });
  
        Ok(token)
      },
      AuthType::OAuth { refresh_token } => {
        unimplemented!()
      }
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum AuthScope {
  Notifications,
  Migration,
}

impl AuthScope {
  pub fn as_str(&self) -> &'static str {
    match *self {
      AuthScope::Notifications => "sellingpartnerapi::notifications",
      AuthScope::Migration => "sellingpartnerapi::migration",
    }
  }
}

#[derive(Debug, Clone)]
enum AuthType {
  OAuth {
    refresh_token: String,
  },
  Grantless {
    scope: &'static str,
  }
}

#[derive(Debug, Clone)]
struct BearerToken {
  token: String,
  expires_at: Instant,
}


#[derive(Debug, Serialize)]
struct GrantRequest<'a> {
  grant_type: &'static str,
  scope: &'static str,
  client_id: &'a str,
  client_secret: &'a str,
}

#[derive(Debug, Deserialize)]
struct ClientCredentialsGrantReply {
  access_token: String,
  expires_in: i32,
}