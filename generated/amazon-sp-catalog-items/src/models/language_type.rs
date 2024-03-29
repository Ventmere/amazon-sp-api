/*
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LanguageType : The language type attribute of an item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LanguageType {
    /// The name attribute of the item.
    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type attribute of the item.
    #[serde(default, rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The audio format attribute of the item.
    #[serde(default, rename = "AudioFormat", skip_serializing_if = "Option::is_none")]
    pub audio_format: Option<String>,
}

impl LanguageType {
    /// The language type attribute of an item.
    pub fn new() -> LanguageType {
        LanguageType {
            name: None,
            _type: None,
            audio_format: None,
        }
    }
}


