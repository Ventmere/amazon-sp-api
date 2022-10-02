use crate::error::SharedError;
use reqwest::Url;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
  pub status: reqwest::StatusCode,
  pub content: String,
  pub entity: Option<T>,
}

pub struct UrlBuilder<E> {
  url: Result<Url, SharedError<E>>,
}

impl<E> UrlBuilder<E> {
  pub fn parse(base: &str) -> Result<Self, SharedError<E>> {
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

  pub fn build(self) -> Result<Url, SharedError<E>> {
    self.url
  }
}
