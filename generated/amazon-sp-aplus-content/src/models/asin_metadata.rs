/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AsinMetadata : The A+ Content ASIN with additional metadata for content management. If you don't include the `includedDataSet` parameter in a call to the listContentDocumentAsinRelations operation, the related ASINs are returned without metadata.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AsinMetadata {
    /// The Amazon Standard Identification Number (ASIN).
    #[serde(default, rename = "asin")]
    pub asin: String,
    /// The set of ASIN badges.
    #[serde(default, rename = "badgeSet", skip_serializing_if = "Option::is_none")]
    pub badge_set: Option<Vec<crate::models::AsinBadge>>,
    /// The Amazon Standard Identification Number (ASIN).
    #[serde(default, rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// The title for the ASIN in the Amazon catalog.
    #[serde(default, rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The default image for the ASIN in the Amazon catalog.
    #[serde(default, rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// A set of content reference keys.
    #[serde(default, rename = "contentReferenceKeySet", skip_serializing_if = "Option::is_none")]
    pub content_reference_key_set: Option<Vec<String>>,
}

impl AsinMetadata {
    /// The A+ Content ASIN with additional metadata for content management. If you don't include the `includedDataSet` parameter in a call to the listContentDocumentAsinRelations operation, the related ASINs are returned without metadata.
    pub fn new(asin: String) -> AsinMetadata {
        AsinMetadata {
            asin,
            badge_set: None,
            parent: None,
            title: None,
            image_url: None,
            content_reference_key_set: None,
        }
    }
}


