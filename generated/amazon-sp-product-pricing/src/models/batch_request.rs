/*
 * Selling Partner API for Pricing
 *
 * The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BatchRequest : Common properties of batch requests against individual APIs.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BatchRequest {
    /// The `getItemOffers` resource path without any query parameters.  **Example:** `/products/pricing/v0/items/B000P6Q7MY/offers`
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "method")]
    pub method: crate::models::HttpMethod,
    /// A mapping of additional HTTP headers to send/receive for the individual batch request.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
}

impl BatchRequest {
    /// Common properties of batch requests against individual APIs.
    pub fn new(uri: String, method: crate::models::HttpMethod) -> BatchRequest {
        BatchRequest {
            uri,
            method,
            headers: None,
        }
    }
}


