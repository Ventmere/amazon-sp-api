/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContentRecord : A content document with additional information for content management.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContentRecord {
    /// A unique reference key for the A+ Content document. A content reference key cannot form a permalink and may change in the future. A content reference key is not guaranteed to match any A+ content identifier.
    #[serde(default, rename = "contentReferenceKey")]
    pub content_reference_key: String,
    #[serde(default, rename = "contentMetadata", skip_serializing_if = "Option::is_none")]
    pub content_metadata: Option<Box<crate::models::ContentMetadata>>,
    #[serde(default, rename = "contentDocument", skip_serializing_if = "Option::is_none")]
    pub content_document: Option<Box<crate::models::ContentDocument>>,
}

impl ContentRecord {
    /// A content document with additional information for content management.
    pub fn new(content_reference_key: String) -> ContentRecord {
        ContentRecord {
            content_reference_key,
            content_metadata: None,
            content_document: None,
        }
    }
}


