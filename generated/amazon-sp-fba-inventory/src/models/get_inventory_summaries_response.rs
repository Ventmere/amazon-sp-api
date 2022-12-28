/*
 * Selling Partner API for FBA Inventory
 *
 * The Selling Partner API for FBA Inventory lets you programmatically retrieve information about inventory in Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetInventorySummariesResponse : The Response schema.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetInventorySummariesResponse {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::GetInventorySummariesResult>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetInventorySummariesResponse {
    /// The Response schema.
    pub fn new() -> GetInventorySummariesResponse {
        GetInventorySummariesResponse {
            payload: None,
            pagination: None,
            errors: None,
        }
    }
}

