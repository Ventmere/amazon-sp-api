/*
 * Selling Partner API for FBA Inventory
 *
 * The Selling Partner API for FBA Inventory lets you programmatically retrieve information about inventory in Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReservedQuantity : The quantity of reserved inventory.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReservedQuantity {
    /// The total number of units in Amazon's fulfillment network that are currently being picked, packed, and shipped; or are sidelined for measurement, sampling, or other internal processes.
    #[serde(rename = "totalReservedQuantity", skip_serializing_if = "Option::is_none")]
    pub total_reserved_quantity: Option<i32>,
    /// The number of units reserved for customer orders.
    #[serde(rename = "pendingCustomerOrderQuantity", skip_serializing_if = "Option::is_none")]
    pub pending_customer_order_quantity: Option<i32>,
    /// The number of units being transferred from one fulfillment center to another.
    #[serde(rename = "pendingTransshipmentQuantity", skip_serializing_if = "Option::is_none")]
    pub pending_transshipment_quantity: Option<i32>,
    /// The number of units that have been sidelined at the fulfillment center for additional processing.
    #[serde(rename = "fcProcessingQuantity", skip_serializing_if = "Option::is_none")]
    pub fc_processing_quantity: Option<i32>,
}

impl ReservedQuantity {
    /// The quantity of reserved inventory.
    pub fn new() -> ReservedQuantity {
        ReservedQuantity {
            total_reserved_quantity: None,
            pending_customer_order_quantity: None,
            pending_transshipment_quantity: None,
            fc_processing_quantity: None,
        }
    }
}


