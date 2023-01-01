/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ErrorList : The error response for when a request is unsuccessful.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorList {
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors")]
    pub errors: Vec<crate::models::Error>,
}

impl ErrorList {
    /// The error response for when a request is unsuccessful.
    pub fn new(errors: Vec<crate::models::Error>) -> ErrorList {
        ErrorList {
            errors,
        }
    }
}


