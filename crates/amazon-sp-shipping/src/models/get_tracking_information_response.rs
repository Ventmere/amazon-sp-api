/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetTrackingInformationResponse : The response schema for the getTrackingInformation operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetTrackingInformationResponse {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::TrackingInformation>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetTrackingInformationResponse {
    /// The response schema for the getTrackingInformation operation.
    pub fn new() -> GetTrackingInformationResponse {
        GetTrackingInformationResponse {
            payload: None,
            errors: None,
        }
    }
}


