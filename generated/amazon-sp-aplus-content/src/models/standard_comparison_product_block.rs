/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StandardComparisonProductBlock : The A+ Content standard comparison product block.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StandardComparisonProductBlock {
    /// The rank or index of this comparison product block within the module. Different blocks cannot occupy the same position within a single module.
    #[serde(default, rename = "position")]
    pub position: i32,
    #[serde(default, rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<crate::models::ImageComponent>>,
    /// The comparison product title.
    #[serde(default, rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The Amazon Standard Identification Number (ASIN).
    #[serde(default, rename = "asin", skip_serializing_if = "Option::is_none")]
    pub asin: Option<String>,
    /// Determines whether this block of content is visually highlighted.
    #[serde(default, rename = "highlight", skip_serializing_if = "Option::is_none")]
    pub highlight: Option<bool>,
    /// Comparison metrics for the product.
    #[serde(default, rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<crate::models::PlainTextItem>>,
}

impl StandardComparisonProductBlock {
    /// The A+ Content standard comparison product block.
    pub fn new(position: i32) -> StandardComparisonProductBlock {
        StandardComparisonProductBlock {
            position,
            image: None,
            title: None,
            asin: None,
            highlight: None,
            metrics: None,
        }
    }
}


