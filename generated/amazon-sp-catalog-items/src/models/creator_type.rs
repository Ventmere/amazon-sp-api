/*
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreatorType : The creator type attribute of an item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatorType {
    /// The value of the attribute.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The role of the value.
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl CreatorType {
    /// The creator type attribute of an item.
    pub fn new() -> CreatorType {
        CreatorType {
            value: None,
            role: None,
        }
    }
}

