/*
 * Selling Partner API for Retail Procurement Transaction Status
 *
 * The Selling Partner API for Retail Procurement Transaction Status provides programmatic access to status information on specific asynchronous POST transactions for vendors.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Error : Error response returned when the request is unsuccessful.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    /// An error code that identifies the type of error that occurred.
    #[serde(rename = "code")]
    pub code: String,
    /// A message that describes the error condition.
    #[serde(rename = "message")]
    pub message: String,
    /// Additional details that can help the caller understand or fix the issue.
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl Error {
    /// Error response returned when the request is unsuccessful.
    pub fn new(code: String, message: String) -> Error {
        Error {
            code,
            message,
            details: None,
        }
    }
}


