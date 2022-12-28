/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContentDocument : The A+ Content document. This is the enhanced content that is published to product detail pages.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContentDocument {
    /// The A+ Content document name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "contentType")]
    pub content_type: crate::models::ContentType,
    /// The A+ Content document subtype. This represents a special-purpose type of an A+ Content document. Not every A+ Content document type will have a subtype, and subtypes may change at any time.
    #[serde(rename = "contentSubType", skip_serializing_if = "Option::is_none")]
    pub content_sub_type: Option<String>,
    /// The IETF language tag. This only supports the primary language subtag with one secondary language subtag. The secondary language subtag is almost always a regional designation. This does not support additional subtags beyond the primary and secondary subtags. **Pattern:** ^[a-z]{2,}-[A-Z0-9]{2,}$
    #[serde(rename = "locale")]
    pub locale: String,
    /// A list of A+ Content modules.
    #[serde(rename = "contentModuleList")]
    pub content_module_list: Vec<crate::models::ContentModule>,
}

impl ContentDocument {
    /// The A+ Content document. This is the enhanced content that is published to product detail pages.
    pub fn new(name: String, content_type: crate::models::ContentType, locale: String, content_module_list: Vec<crate::models::ContentModule>) -> ContentDocument {
        ContentDocument {
            name,
            content_type,
            content_sub_type: None,
            locale,
            content_module_list,
        }
    }
}

