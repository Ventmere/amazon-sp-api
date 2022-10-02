use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("unknown sp api endpoint: {0}")]
  UnknownSpApiEndpoint(String),
}