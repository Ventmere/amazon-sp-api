/*
 * Selling Partner API for Feeds
 *
 * The Selling Partner API for Feeds lets you upload data to Amazon on behalf of a selling partner.
 *
 * The version of the OpenAPI document: 2020-09-04
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFeedSpecification {
    /// The feed type.
    #[serde(rename = "feedType")]
    pub feed_type: String,
    /// A list of identifiers for marketplaces that you want the feed to be applied to.
    #[serde(rename = "marketplaceIds")]
    pub marketplace_ids: Vec<String>,
    /// The document identifier returned by the createFeedDocument operation. Encrypt and upload the feed document contents before calling the createFeed operation.
    #[serde(rename = "inputFeedDocumentId")]
    pub input_feed_document_id: String,
    /// Additional options to control the feed. For feeds that use the feedOptions parameter, you can find the parameter values in the feed description in [feedType values](doc:feed-type-values).
    #[serde(rename = "feedOptions", skip_serializing_if = "Option::is_none")]
    pub feed_options: Option<::std::collections::HashMap<String, String>>,
}

impl CreateFeedSpecification {
    pub fn new(feed_type: String, marketplace_ids: Vec<String>, input_feed_document_id: String) -> CreateFeedSpecification {
        CreateFeedSpecification {
            feed_type,
            marketplace_ids,
            input_feed_document_id,
            feed_options: None,
        }
    }
}

