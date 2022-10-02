/*
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SubmitAcknowledgementRequest : The request schema for the submitAcknowledgement operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitAcknowledgementRequest {
    /// A list of one or more purchase orders.
    #[serde(rename = "orderAcknowledgements", skip_serializing_if = "Option::is_none")]
    pub order_acknowledgements: Option<Vec<crate::models::OrderAcknowledgementItem>>,
}

impl SubmitAcknowledgementRequest {
    /// The request schema for the submitAcknowledgement operation.
    pub fn new() -> SubmitAcknowledgementRequest {
        SubmitAcknowledgementRequest {
            order_acknowledgements: None,
        }
    }
}

