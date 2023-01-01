/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValidateContentDocumentAsinRelationsResponse {
    /// A set of messages to the user, such as warnings or comments.
    #[serde(default, rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<crate::models::Error>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors")]
    pub errors: Vec<crate::models::Error>,
}

impl ValidateContentDocumentAsinRelationsResponse {
    pub fn new(errors: Vec<crate::models::Error>) -> ValidateContentDocumentAsinRelationsResponse {
        ValidateContentDocumentAsinRelationsResponse {
            warnings: None,
            errors,
        }
    }
}


