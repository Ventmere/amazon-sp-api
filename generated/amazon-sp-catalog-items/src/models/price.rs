/*
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Price : The price attribute of the item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Price {
    /// The amount.
    #[serde(default, rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// The currency code of the amount.
    #[serde(default, rename = "CurrencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

impl Price {
    /// The price attribute of the item.
    pub fn new() -> Price {
        Price {
            amount: None,
            currency_code: None,
        }
    }
}


