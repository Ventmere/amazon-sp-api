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
pub struct SearchContentPublishRecordsResponse {
    /// A set of messages to the user, such as warnings or comments.
    #[serde(default, rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<crate::models::Error>>,
    /// A page token that is returned when the results of the call exceed the page size. To get another page of results, call the operation again, passing in this value with the pageToken parameter.
    #[serde(default, rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// A list of A+ Content publishing records.
    #[serde(default, rename = "publishRecordList")]
    pub publish_record_list: Vec<crate::models::PublishRecord>,
}

impl SearchContentPublishRecordsResponse {
    pub fn new(publish_record_list: Vec<crate::models::PublishRecord>) -> SearchContentPublishRecordsResponse {
        SearchContentPublishRecordsResponse {
            warnings: None,
            next_page_token: None,
            publish_record_list,
        }
    }
}


