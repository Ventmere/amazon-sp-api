/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TaxClassification : The tax classification for the order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxClassification {
    /// The type of tax.
    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The buyer's tax identifier.
    #[serde(default, rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl TaxClassification {
    /// The tax classification for the order.
    pub fn new() -> TaxClassification {
        TaxClassification {
            name: None,
            value: None,
        }
    }
}


