/*
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Error : Error response returned when the request is unsuccessful.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    /// An error code that identifies the type of error that occurred.
    #[serde(rename = "code")]
    pub code: String,
    /// A message that describes the error condition in a human-readable form.
    #[serde(rename = "message")]
    pub message: String,
    /// Additional information that can help the caller understand or fix the issue.
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


