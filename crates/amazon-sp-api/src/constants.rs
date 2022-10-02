use std::str::FromStr;
use crate::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum SpApiEndpoint {
  NorthAmerica,
  Europe,
  FarEast,
}

impl SpApiEndpoint {
  pub fn get_url(&self) -> &'static str {
    match *self {
      SpApiEndpoint::NorthAmerica => "https://sellingpartnerapi-na.amazon.com",
      SpApiEndpoint::Europe => "https://sellingpartnerapi-eu.amazon.com",
      SpApiEndpoint::FarEast => "https://sellingpartnerapi-fe.amazon.com",
    }
  }

  pub fn get_aws_region(&self) -> &'static str {
    match *self {
      SpApiEndpoint::NorthAmerica => "us-east-1",
      SpApiEndpoint::Europe => "eu-west-1",
      SpApiEndpoint::FarEast => "us-west-2",
    }
  }
}

impl FromStr for SpApiEndpoint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "us-east-1" => Ok(SpApiEndpoint::NorthAmerica),
        "eu-west-1" => Ok(SpApiEndpoint::Europe),
        "us-west-2" => Ok(SpApiEndpoint::FarEast),
        other => Err(Error::UnknownSpApiEndpoint(other.to_string()))
      }
    }
}