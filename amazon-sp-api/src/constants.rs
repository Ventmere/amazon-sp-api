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
}