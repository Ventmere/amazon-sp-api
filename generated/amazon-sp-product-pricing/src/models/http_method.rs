/*
 * Selling Partner API for Pricing
 *
 * The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HttpMethod : The HTTP method associated with the individual APIs being called as part of the batch request.

/// The HTTP method associated with the individual APIs being called as part of the batch request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "POST")]
    POST,

}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            Self::GET => String::from("GET"),
            Self::PUT => String::from("PUT"),
            Self::PATCH => String::from("PATCH"),
            Self::DELETE => String::from("DELETE"),
            Self::POST => String::from("POST"),
        }
    }
}

impl Default for HttpMethod {
    fn default() -> HttpMethod {
        Self::GET
    }
}




