/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StandardImageSidebarModule : Two images, two paragraphs, and two bulleted lists. One image is smaller and displayed in the sidebar.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StandardImageSidebarModule {
    #[serde(rename = "headline", skip_serializing_if = "Option::is_none")]
    pub headline: Option<Box<crate::models::TextComponent>>,
    #[serde(rename = "imageCaptionBlock", skip_serializing_if = "Option::is_none")]
    pub image_caption_block: Option<Box<crate::models::StandardImageCaptionBlock>>,
    #[serde(rename = "descriptionTextBlock", skip_serializing_if = "Option::is_none")]
    pub description_text_block: Option<Box<crate::models::StandardTextBlock>>,
    #[serde(rename = "descriptionListBlock", skip_serializing_if = "Option::is_none")]
    pub description_list_block: Option<Box<crate::models::StandardTextListBlock>>,
    #[serde(rename = "sidebarImageTextBlock", skip_serializing_if = "Option::is_none")]
    pub sidebar_image_text_block: Option<Box<crate::models::StandardImageTextBlock>>,
    #[serde(rename = "sidebarListBlock", skip_serializing_if = "Option::is_none")]
    pub sidebar_list_block: Option<Box<crate::models::StandardTextListBlock>>,
}

impl StandardImageSidebarModule {
    /// Two images, two paragraphs, and two bulleted lists. One image is smaller and displayed in the sidebar.
    pub fn new() -> StandardImageSidebarModule {
        StandardImageSidebarModule {
            headline: None,
            image_caption_block: None,
            description_text_block: None,
            description_list_block: None,
            sidebar_image_text_block: None,
            sidebar_list_block: None,
        }
    }
}


