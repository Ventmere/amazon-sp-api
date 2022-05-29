/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CompleteServiceJobByServiceJobIdResponse : Response schema for CompleteServiceJobByServiceJobId operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CompleteServiceJobByServiceJobIdResponse {
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl CompleteServiceJobByServiceJobIdResponse {
    /// Response schema for CompleteServiceJobByServiceJobId operation.
    pub fn new() -> CompleteServiceJobByServiceJobIdResponse {
        CompleteServiceJobByServiceJobIdResponse {
            errors: None,
        }
    }
}


