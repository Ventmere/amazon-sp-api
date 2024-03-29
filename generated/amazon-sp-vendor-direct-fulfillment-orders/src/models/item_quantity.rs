/*
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemQuantity : Details of quantity ordered.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemQuantity {
    /// Acknowledged quantity. This value should not be zero.
    #[serde(default, rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// Unit of measure for the acknowledged quantity.
    #[serde(default, rename = "unitOfMeasure", skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<UnitOfMeasure>,
}

impl ItemQuantity {
    /// Details of quantity ordered.
    pub fn new() -> ItemQuantity {
        ItemQuantity {
            amount: None,
            unit_of_measure: None,
        }
    }
}

/// Unit of measure for the acknowledged quantity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnitOfMeasure {
    #[serde(rename = "Each")]
    Each,
}

impl Default for UnitOfMeasure {
    fn default() -> UnitOfMeasure {
        Self::Each
    }
}

