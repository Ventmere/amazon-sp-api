/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SubmitShipmentConfirmationsResponse : The response schema for the submitShipmentConfirmations operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitShipmentConfirmationsResponse {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::TransactionReference>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl SubmitShipmentConfirmationsResponse {
    /// The response schema for the submitShipmentConfirmations operation.
    pub fn new() -> SubmitShipmentConfirmationsResponse {
        SubmitShipmentConfirmationsResponse {
            payload: None,
            errors: None,
        }
    }
}

