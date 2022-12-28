/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// ErrorList : A list of error responses returned when a request is unsuccessful.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorList {
    #[serde(rename = "errors")]
    pub errors: Vec<crate::models::Error>,
}

impl ErrorList {
    /// A list of error responses returned when a request is unsuccessful.
    pub fn new(errors: Vec<crate::models::Error>) -> ErrorList {
        ErrorList {
            errors,
        }
    }
}

