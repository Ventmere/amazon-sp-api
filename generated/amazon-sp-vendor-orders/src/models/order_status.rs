/*
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderStatus : Current status of a purchase order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderStatus {
    /// The buyer's purchase order number for this order. Formatting Notes: 8-character alpha-numeric code.
    #[serde(rename = "purchaseOrderNumber")]
    pub purchase_order_number: String,
    /// The status of the buyer's purchase order for this order.
    #[serde(rename = "purchaseOrderStatus")]
    pub purchase_order_status: PurchaseOrderStatus,
    /// The date the purchase order was placed. Must be in ISO-8601 date/time format.
    #[serde(rename = "purchaseOrderDate")]
    pub purchase_order_date: String,
    /// The date when the purchase order was last updated. Must be in ISO-8601 date/time format.
    #[serde(rename = "lastUpdatedDate", skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    #[serde(rename = "sellingParty")]
    pub selling_party: Box<crate::models::PartyIdentification>,
    #[serde(rename = "shipToParty")]
    pub ship_to_party: Box<crate::models::PartyIdentification>,
    /// Detailed description of items order status.
    #[serde(rename = "itemStatus")]
    pub item_status: Vec<crate::models::OrderItemStatus>,
}

impl OrderStatus {
    /// Current status of a purchase order.
    pub fn new(purchase_order_number: String, purchase_order_status: PurchaseOrderStatus, purchase_order_date: String, selling_party: crate::models::PartyIdentification, ship_to_party: crate::models::PartyIdentification, item_status: Vec<crate::models::OrderItemStatus>) -> OrderStatus {
        OrderStatus {
            purchase_order_number,
            purchase_order_status,
            purchase_order_date,
            last_updated_date: None,
            selling_party: Box::new(selling_party),
            ship_to_party: Box::new(ship_to_party),
            item_status,
        }
    }
}

/// The status of the buyer's purchase order for this order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PurchaseOrderStatus {
    #[serde(rename = "OPEN")]
    OPEN,
    #[serde(rename = "CLOSED")]
    CLOSED,
}

impl Default for PurchaseOrderStatus {
    fn default() -> PurchaseOrderStatus {
        Self::OPEN
    }
}
