/*
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Image : The image attribute of the item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Image {
    /// The image URL attribute of the item.
    #[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
    pub URL: Option<String>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<Box<crate::models::DecimalWithUnits>>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<Box<crate::models::DecimalWithUnits>>,
}

impl Image {
    /// The image attribute of the item.
    pub fn new() -> Image {
        Image {
            URL: None,
            height: None,
            width: None,
        }
    }
}

