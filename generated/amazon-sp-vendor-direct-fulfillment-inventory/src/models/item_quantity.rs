/*
 * Selling Partner API for Direct Fulfillment Inventory Updates
 *
 * The Selling Partner API for Direct Fulfillment Inventory Updates provides programmatic access to a direct fulfillment vendor's inventory updates.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemQuantity : Details of item quantity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemQuantity {
    /// Quantity of units available for a specific item.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// Unit of measure for the available quantity.
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: String,
}

impl ItemQuantity {
    /// Details of item quantity.
    pub fn new(unit_of_measure: String) -> ItemQuantity {
        ItemQuantity {
            amount: None,
            unit_of_measure,
        }
    }
}


