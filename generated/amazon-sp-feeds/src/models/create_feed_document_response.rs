/*
 * Selling Partner API for Feeds
 *
 * The Selling Partner API for Feeds lets you upload data to Amazon on behalf of a selling partner.
 *
 * The version of the OpenAPI document: 2020-09-04
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateFeedDocumentResponse : The response for the createFeedDocument operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFeedDocumentResponse {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::CreateFeedDocumentResult>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl CreateFeedDocumentResponse {
    /// The response for the createFeedDocument operation.
    pub fn new() -> CreateFeedDocumentResponse {
        CreateFeedDocumentResponse {
            payload: None,
            errors: None,
        }
    }
}

