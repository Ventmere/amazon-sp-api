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
  token: Arc<RwLock<Option<Token>>>,
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

  pub fn oauth_code(client_id: String, client_secret: String, code: String) -> Self {
    Self {
      client_id,
      client_secret,
      auth_type: AuthType::OAuthCode { 
        code
      },
      token: Arc::new(RwLock::new(None)),
    }
  }

  pub fn oauth_refresh_token(client_id: String, client_secret: String, refresh_token: String) -> Self {
    Self {
      client_id,
      client_secret,
      auth_type: AuthType::OAuthRefreshToken {
        refresh_token
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
        return Ok(token.access_token)
      }
      self.token.write().take();
    }

    match &self.auth_type {
      AuthType::Grantless { scope } => {
        tracing::debug!("getting token using client_credentials");
        let req = GrantRequest {
          grant_type: "client_credentials",
          client_id: &self.client_id,
          client_secret: &self.client_secret,
          scope: Some(scope),
          code: None,
          refresh_token: None,
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
  
        let access_token = reply.access_token;
        let expires_at = Instant::now() + Duration::from_secs(reply.expires_in as _);

        tracing::debug!("token = {}, expires_in = {}", access_token, reply.expires_in);

        self.token.write().replace(Token {
          access_token: access_token.clone(),
          expires_at,
          refresh_token: None,
        });
  
        Ok(access_token)
      },
      AuthType::OAuthCode { code } => {
        tracing::debug!("getting token using authorization_code");
        let req = GrantRequest {
          grant_type: "authorization_code",
          client_id: &self.client_id,
          client_secret: &self.client_secret,
          scope: None,
          code: Some(&code),
          refresh_token: None,
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
  
        let reply: AuthorizationCodeGrantReply = if let Some(ref body ) = body {
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
  
        let access_token = reply.access_token;
        let expires_at = Instant::now() + Duration::from_secs(reply.expires_in as _);
        let refresh_token = reply.refresh_token;

        tracing::debug!("token = {}, expires_in = {}, refresh_token = {}", access_token, reply.expires_in, refresh_token);

        self.token.write().replace(Token {
          access_token: access_token.clone(),
          expires_at,
          refresh_token: Some(refresh_token),
        });
  
        Ok(access_token)
      },
      AuthType::OAuthRefreshToken { refresh_token } => {
        tracing::debug!("getting token using refresh_token");
        let req = GrantRequest {
          grant_type: "refresh_token",
          client_id: &self.client_id,
          client_secret: &self.client_secret,
          scope: None,
          code: None,
          refresh_token: Some(refresh_token),
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
  
        let reply: AuthorizationCodeGrantReply = if let Some(ref body ) = body {
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
  
        let access_token = reply.access_token;
        let expires_at = Instant::now() + Duration::from_secs(reply.expires_in as _);
        let refresh_token = reply.refresh_token;

        tracing::debug!("token = {}, expires_in = {}, refresh_token = {}", access_token, reply.expires_in, refresh_token);

        self.token.write().replace(Token {
          access_token: access_token.clone(),
          expires_at,
          refresh_token: Some(refresh_token),
        });
  
        Ok(access_token)
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
  OAuthCode {
    code: String,
  },
  OAuthRefreshToken {
    refresh_token: String,
  },
  Grantless {
    scope: &'static str,
  }
}

#[derive(Debug, Clone)]
struct Token {
  access_token: String,
  expires_at: Instant,
  refresh_token: Option<String>,
}


#[derive(Debug, Serialize)]
struct GrantRequest<'a> {
  grant_type: &'static str,
  client_id: &'a str,
  client_secret: &'a str,
  scope: Option<&'static str>,
  code: Option<&'a str>,
  refresh_token: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct ClientCredentialsGrantReply {
  access_token: String,
  expires_in: i32,
}

#[derive(Debug, Deserialize)]
struct AuthorizationCodeGrantReply {
  access_token: String,
  token_type: String,
  expires_in: i32,
  refresh_token: String,
} 