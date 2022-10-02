use thiserror::Error;
use aws_sigv4;
use crate::request::ResponseContent;

#[derive(Error, Debug)]
pub enum SharedError<T = ()> {  
  #[error("reqwest: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("serde: {0}")]
  Serde(#[from] serde_json::Error),
  #[error("IO: {0}")]
  Io(#[from] std::io::Error),
  #[error("response: {0:?}")]
  ResponseError(ResponseContent<T>),
  #[error("aws v4 signature: {0}")]
  AWSV4SignatureError(#[from] aws_sigv4::http_request::Error),
  #[error("url parse: {0}")]
  UrlParse(#[from] url::ParseError),
  #[error("url encode: {0}")]
  UrlEncode(#[from] serde_urlencoded::ser::Error),
}