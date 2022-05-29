/*
 * Selling Partner API for Feeds
 *
 * The Selling Partner API for Feeds lets you upload data to Amazon on behalf of a selling partner.
 *
 * The version of the OpenAPI document: 2021-06-30
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Feed : Detailed information about the feed.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Feed {
    /// The identifier for the feed. This identifier is unique only in combination with a seller ID.
    #[serde(rename = "feedId")]
    pub feed_id: String,
    /// The feed type.
    #[serde(rename = "feedType")]
    pub feed_type: String,
    /// A list of identifiers for the marketplaces that the feed is applied to.
    #[serde(rename = "marketplaceIds", skip_serializing_if = "Option::is_none")]
    pub marketplace_ids: Option<Vec<String>>,
    /// The date and time when the feed was created, in ISO 8601 date time format.
    #[serde(rename = "createdTime")]
    pub created_time: String,
    /// The processing status of the feed.
    #[serde(rename = "processingStatus")]
    pub processing_status: ProcessingStatus,
    /// The date and time when feed processing started, in ISO 8601 date time format.
    #[serde(rename = "processingStartTime", skip_serializing_if = "Option::is_none")]
    pub processing_start_time: Option<String>,
    /// The date and time when feed processing completed, in ISO 8601 date time format.
    #[serde(rename = "processingEndTime", skip_serializing_if = "Option::is_none")]
    pub processing_end_time: Option<String>,
    /// The identifier for the feed document. This identifier is unique only in combination with a seller ID.
    #[serde(rename = "resultFeedDocumentId", skip_serializing_if = "Option::is_none")]
    pub result_feed_document_id: Option<String>,
}

impl Feed {
    /// Detailed information about the feed.
    pub fn new(feed_id: String, feed_type: String, created_time: String, processing_status: ProcessingStatus) -> Feed {
        Feed {
            feed_id,
            feed_type,
            marketplace_ids: None,
            created_time,
            processing_status,
            processing_start_time: None,
            processing_end_time: None,
            result_feed_document_id: None,
        }
    }
}

/// The processing status of the feed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProcessingStatus {
    #[serde(rename = "CANCELLED")]
    CANCELLED,
    #[serde(rename = "DONE")]
    DONE,
    #[serde(rename = "FATAL")]
    FATAL,
    #[serde(rename = "IN_PROGRESS")]
    INPROGRESS,
    #[serde(rename = "IN_QUEUE")]
    INQUEUE,
}

impl Default for ProcessingStatus {
    fn default() -> ProcessingStatus {
        Self::CANCELLED
    }
}

