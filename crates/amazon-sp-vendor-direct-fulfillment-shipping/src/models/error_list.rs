/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: 2021-12-28
 * 
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


