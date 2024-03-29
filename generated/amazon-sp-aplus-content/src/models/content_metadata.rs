/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContentMetadata : The metadata of an A+ Content document.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContentMetadata {
    /// The A+ Content document name.
    #[serde(default, rename = "name")]
    pub name: String,
    /// The identifier for the marketplace where the A+ Content is published.
    #[serde(default, rename = "marketplaceId")]
    pub marketplace_id: String,
    #[serde(default, rename = "status")]
    pub status: crate::models::ContentStatus,
    /// The set of content badges.
    #[serde(default, rename = "badgeSet")]
    pub badge_set: Vec<crate::models::ContentBadge>,
    /// The approximate age of the A+ Content document and metadata.
    #[serde(default, rename = "updateTime")]
    pub update_time: String,
}

impl ContentMetadata {
    /// The metadata of an A+ Content document.
    pub fn new(name: String, marketplace_id: String, status: crate::models::ContentStatus, badge_set: Vec<crate::models::ContentBadge>, update_time: String) -> ContentMetadata {
        ContentMetadata {
            name,
            marketplace_id,
            status,
            badge_set,
            update_time,
        }
    }
}


