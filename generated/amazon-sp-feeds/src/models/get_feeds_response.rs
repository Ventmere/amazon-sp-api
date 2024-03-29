/*
 * Selling Partner API for Feeds
 *
 * The Selling Partner API for Feeds lets you upload data to Amazon on behalf of a selling partner.
 *
 * The version of the OpenAPI document: 2020-09-04
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFeedsResponse : Response schema.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFeedsResponse {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<crate::models::Feed>>,
    /// Returned when the number of results exceeds pageSize. To get the next page of results, call the getFeeds operation with this token as the only parameter.
    #[serde(default, rename = "nextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetFeedsResponse {
    /// Response schema.
    pub fn new() -> GetFeedsResponse {
        GetFeedsResponse {
            payload: None,
            next_token: None,
            errors: None,
        }
    }
}


