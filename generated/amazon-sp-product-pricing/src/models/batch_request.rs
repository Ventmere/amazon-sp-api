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
    /// The resource path of the operation you are calling in batch without any query parameters.  If you are calling `getItemOffersBatch`, supply the path of `getItemOffers`.  **Example:** `/products/pricing/v0/items/B000P6Q7MY/offers`  If you are calling `getListingOffersBatch`, supply the path of `getListingOffers`.  **Example:** `/products/pricing/v0/listings/B000P6Q7MY/offers`
    #[serde(default, rename = "uri")]
    pub uri: String,
    #[serde(default, rename = "method")]
    pub method: crate::models::HttpMethod,
    /// A mapping of additional HTTP headers to send/receive for the individual batch request.
    #[serde(default, rename = "headers", skip_serializing_if = "Option::is_none")]
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


