pub use crate::request::ResponseError;
use aws_sigv4;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SharedError {
  #[error("reqwest: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("serde: {0}")]
  Serde(#[from] serde_json::Error),
  #[error("IO: {0}")]
  Io(#[from] std::io::Error),
  #[error("response: {0:?}")]
  ResponseError(ResponseError),
  #[error("aws v4 signature: {0}")]
  AWSV4SignatureError(#[from] aws_sigv4::http_request::Error),
  #[error("url parse: {0}")]
  UrlParse(#[from] url::ParseError),
  #[error("url encode: {0}")]
  UrlEncode(#[from] serde_urlencoded::ser::Error),
  #[error("Expired restricted data token")]
  ExpiredRDT,
}

impl SharedError
{
  pub fn is_retryable(&self) -> bool {
    match *self {
      Self::ResponseError(ref response) => {
        if response.status.is_server_error() {
          return true;
        }
        match response.status {
          reqwest::StatusCode::TOO_MANY_REQUESTS => true,
          _ => false,
        }
      }
      _ => false,
    }
  }
}
