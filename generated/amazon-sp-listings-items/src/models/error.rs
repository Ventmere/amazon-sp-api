/*
 * Selling Partner API for Listings Items
 *
 * The Selling Partner API for Listings Items (Listings Items API) provides programmatic access to selling partner listings on Amazon. Use this API in collaboration with the Selling Partner API for Product Type Definitions, which you use to retrieve the information about Amazon product types needed to use the Listings Items API.  For more information, see the [Listing Items API Use Case Guide](doc:listings-items-api-v2020-09-01-use-case-guide).
 *
 * The version of the OpenAPI document: 2020-09-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Error : Error response returned when the request is unsuccessful.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    /// An error code that identifies the type of error that occurred.
    #[serde(default, rename = "code")]
    pub code: String,
    /// A message that describes the error condition.
    #[serde(default, rename = "message")]
    pub message: String,
    /// Additional details that can help the caller understand or fix the issue.
    #[serde(default, rename = "details", skip_serializing_if = "Option::is_none")]
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


