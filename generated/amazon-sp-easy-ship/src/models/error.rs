/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
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


