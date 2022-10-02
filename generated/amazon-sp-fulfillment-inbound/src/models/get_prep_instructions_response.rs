/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetPrepInstructionsResponse : The response schema for the getPrepInstructions operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetPrepInstructionsResponse {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::GetPrepInstructionsResult>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetPrepInstructionsResponse {
    /// The response schema for the getPrepInstructions operation.
    pub fn new() -> GetPrepInstructionsResponse {
        GetPrepInstructionsResponse {
            payload: None,
            errors: None,
        }
    }
}

