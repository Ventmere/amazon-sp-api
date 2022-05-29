/*
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * The version of the OpenAPI document: 2021-12-28
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SubmitAcknowledgementResponse : The response schema for the submitAcknowledgement operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitAcknowledgementResponse {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::TransactionId>>,
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Box<crate::models::ErrorList>>,
}

impl SubmitAcknowledgementResponse {
    /// The response schema for the submitAcknowledgement operation.
    pub fn new() -> SubmitAcknowledgementResponse {
        SubmitAcknowledgementResponse {
            payload: None,
            errors: None,
        }
    }
}


