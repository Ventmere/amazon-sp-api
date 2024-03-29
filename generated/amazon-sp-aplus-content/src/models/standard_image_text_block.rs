/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StandardImageTextBlock : The A+ Content standard image and text box block.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StandardImageTextBlock {
    #[serde(default, rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<crate::models::ImageComponent>>,
    #[serde(default, rename = "headline", skip_serializing_if = "Option::is_none")]
    pub headline: Option<Box<crate::models::TextComponent>>,
    #[serde(default, rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<crate::models::ParagraphComponent>>,
}

impl StandardImageTextBlock {
    /// The A+ Content standard image and text box block.
    pub fn new() -> StandardImageTextBlock {
        StandardImageTextBlock {
            image: None,
            headline: None,
            body: None,
        }
    }
}


