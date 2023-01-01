/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CancelShipmentResponse : The response schema for the cancelShipment operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CancelShipmentResponse {
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl CancelShipmentResponse {
    /// The response schema for the cancelShipment operation.
    pub fn new() -> CancelShipmentResponse {
        CancelShipmentResponse {
            errors: None,
        }
    }
}


