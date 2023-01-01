/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetOrderBuyerInfoResponse : The response schema for the getOrderBuyerInfo operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOrderBuyerInfoResponse {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::OrderBuyerInfo>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetOrderBuyerInfoResponse {
    /// The response schema for the getOrderBuyerInfo operation.
    pub fn new() -> GetOrderBuyerInfoResponse {
        GetOrderBuyerInfoResponse {
            payload: None,
            errors: None,
        }
    }
}


