/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetServiceJobsResponse : Response schema for the `getServiceJobs` operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetServiceJobsResponse {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::JobListing>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetServiceJobsResponse {
    /// Response schema for the `getServiceJobs` operation.
    pub fn new() -> GetServiceJobsResponse {
        GetServiceJobsResponse {
            payload: None,
            errors: None,
        }
    }
}

