/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StandardHeaderTextListBlock : The A+ standard fixed-length list of text, with a related headline.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StandardHeaderTextListBlock {
    #[serde(default, rename = "headline", skip_serializing_if = "Option::is_none")]
    pub headline: Option<Box<crate::models::TextComponent>>,
    #[serde(default, rename = "block", skip_serializing_if = "Option::is_none")]
    pub block: Option<Box<crate::models::StandardTextListBlock>>,
}

impl StandardHeaderTextListBlock {
    /// The A+ standard fixed-length list of text, with a related headline.
    pub fn new() -> StandardHeaderTextListBlock {
        StandardHeaderTextListBlock {
            headline: None,
            block: None,
        }
    }
}


