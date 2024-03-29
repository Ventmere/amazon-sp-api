/*
 * Selling Partner API for Retail Procurement Payments
 *
 * The Selling Partner API for Retail Procurement Payments provides programmatic access to vendors payments data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemQuantity : Details of quantity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemQuantity {
    /// Quantity of an item. This value should not be zero.
    #[serde(default, rename = "amount")]
    pub amount: i32,
    /// Unit of measure for the quantity.
    #[serde(default, rename = "unitOfMeasure")]
    pub unit_of_measure: UnitOfMeasure,
    /// The case size, if the unit of measure value is Cases.
    #[serde(default, rename = "unitSize", skip_serializing_if = "Option::is_none")]
    pub unit_size: Option<i32>,
}

impl ItemQuantity {
    /// Details of quantity.
    pub fn new(amount: i32, unit_of_measure: UnitOfMeasure) -> ItemQuantity {
        ItemQuantity {
            amount,
            unit_of_measure,
            unit_size: None,
        }
    }
}

/// Unit of measure for the quantity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnitOfMeasure {
    #[serde(rename = "Cases")]
    Cases,
    #[serde(rename = "Eaches")]
    Eaches,
}

impl Default for UnitOfMeasure {
    fn default() -> UnitOfMeasure {
        Self::Cases
    }
}

