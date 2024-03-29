/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StandardTextPairBlock : The A+ Content standard label and description block, comprised of a pair of text components.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StandardTextPairBlock {
    #[serde(default, rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<crate::models::TextComponent>>,
    #[serde(default, rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<crate::models::TextComponent>>,
}

impl StandardTextPairBlock {
    /// The A+ Content standard label and description block, comprised of a pair of text components.
    pub fn new() -> StandardTextPairBlock {
        StandardTextPairBlock {
            label: None,
            description: None,
        }
    }
}


