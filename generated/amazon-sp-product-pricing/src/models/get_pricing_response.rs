/*
 * Selling Partner API for Pricing
 *
 * The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetPricingResponse : The response schema for the `getPricing` and `getCompetitivePricing` operations.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetPricingResponse {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<crate::models::Price>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetPricingResponse {
    /// The response schema for the `getPricing` and `getCompetitivePricing` operations.
    pub fn new() -> GetPricingResponse {
        GetPricingResponse {
            payload: None,
            errors: None,
        }
    }
}

