use crate::error::SharedError;
use reqwest::Url;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct ResponseError {
  pub status: reqwest::StatusCode,
  pub content: String,
  pub error_list: Option<Vec<Error>>,
}

pub struct UrlBuilder {
  url: Result<Url, SharedError>,
}

impl UrlBuilder {
  pub fn parse(base: &str) -> Result<Self, SharedError> {
    Ok(Self {
      url: Ok(Url::parse(base)?),
    })
  }

  pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
    let mut err = None;
    if let Ok(ref mut url) = self.url {
      let mut pairs = url.query_pairs_mut();
      let serializer = serde_urlencoded::Serializer::new(&mut pairs);
      if let Err(e) = query.serialize(serializer) {
        err.replace(e.into());
      }
    }
    if let Some(err) = err {
      self.url = Err(err);
    }
    self
  }

  pub fn build(self) -> Result<Url, SharedError> {
    self.url
  }
}


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    /// The code that identifies the type of error condition.
    #[serde(rename = "code")]
    pub code: String,
    /// A human readable description of the error condition.
    #[serde(rename = "message")]
    pub message: String,
    /// Additional information, if available, to clarify the error condition.
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl Error {
    /// Error response returned when the request is unsuccessful.
    pub fn new(code: String, message: String) -> Error {
        Error {
            code,
            message,
            details: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorList {
    /// A list of error responses returned when the request is unsuccessful.
    pub errors: Vec<Error>,
}