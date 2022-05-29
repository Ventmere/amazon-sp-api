/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StandardSingleImageHighlightsModule : A standard image with several paragraphs and a bulleted list.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StandardSingleImageHighlightsModule {
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<crate::models::ImageComponent>>,
    #[serde(rename = "headline", skip_serializing_if = "Option::is_none")]
    pub headline: Option<Box<crate::models::TextComponent>>,
    #[serde(rename = "textBlock1", skip_serializing_if = "Option::is_none")]
    pub text_block1: Option<Box<crate::models::StandardTextBlock>>,
    #[serde(rename = "textBlock2", skip_serializing_if = "Option::is_none")]
    pub text_block2: Option<Box<crate::models::StandardTextBlock>>,
    #[serde(rename = "textBlock3", skip_serializing_if = "Option::is_none")]
    pub text_block3: Option<Box<crate::models::StandardTextBlock>>,
    #[serde(rename = "bulletedListBlock", skip_serializing_if = "Option::is_none")]
    pub bulleted_list_block: Option<Box<crate::models::StandardHeaderTextListBlock>>,
}

impl StandardSingleImageHighlightsModule {
    /// A standard image with several paragraphs and a bulleted list.
    pub fn new() -> StandardSingleImageHighlightsModule {
        StandardSingleImageHighlightsModule {
            image: None,
            headline: None,
            text_block1: None,
            text_block2: None,
            text_block3: None,
            bulleted_list_block: None,
        }
    }
}


