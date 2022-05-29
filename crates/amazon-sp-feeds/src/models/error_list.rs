/*
 * Selling Partner API for Feeds
 *
 * The Selling Partner API for Feeds lets you upload data to Amazon on behalf of a selling partner.
 *
 * The version of the OpenAPI document: 2021-06-30
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


