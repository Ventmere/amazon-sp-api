/*
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetPurchaseOrdersStatusResponse : The response schema for the getPurchaseOrdersStatus operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetPurchaseOrdersStatusResponse {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::OrderListStatus>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetPurchaseOrdersStatusResponse {
    /// The response schema for the getPurchaseOrdersStatus operation.
    pub fn new() -> GetPurchaseOrdersStatusResponse {
        GetPurchaseOrdersStatusResponse {
            payload: None,
            errors: None,
        }
    }
}


