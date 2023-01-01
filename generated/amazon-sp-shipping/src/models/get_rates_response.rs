/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetRatesResponse : The response schema for the getRates operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetRatesResponse {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::GetRatesResult>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetRatesResponse {
    /// The response schema for the getRates operation.
    pub fn new() -> GetRatesResponse {
        GetRatesResponse {
            payload: None,
            errors: None,
        }
    }
}


