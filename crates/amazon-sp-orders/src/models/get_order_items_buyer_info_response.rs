/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetOrderItemsBuyerInfoResponse : The response schema for the getOrderItemsBuyerInfo operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOrderItemsBuyerInfoResponse {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::OrderItemsBuyerInfoList>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetOrderItemsBuyerInfoResponse {
    /// The response schema for the getOrderItemsBuyerInfo operation.
    pub fn new() -> GetOrderItemsBuyerInfoResponse {
        GetOrderItemsBuyerInfoResponse {
            payload: None,
            errors: None,
        }
    }
}


