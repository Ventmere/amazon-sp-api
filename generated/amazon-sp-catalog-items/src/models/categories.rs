/*
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Categories {
    /// The identifier for the product category (or browse node).
    #[serde(default, rename = "ProductCategoryId", skip_serializing_if = "Option::is_none")]
    pub product_category_id: Option<String>,
    /// The name of the product category (or browse node).
    #[serde(default, rename = "ProductCategoryName", skip_serializing_if = "Option::is_none")]
    pub product_category_name: Option<String>,
    /// The parent product category.
    #[serde(default, rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<serde_json::Value>,
}

impl Categories {
    pub fn new() -> Categories {
        Categories {
            product_category_id: None,
            product_category_name: None,
            parent: None,
        }
    }
}


