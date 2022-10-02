/*
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderItemStatus {
    /// Numbering of the item on the purchase order. The first item will be 1, the second 2, and so on.
    #[serde(rename = "itemSequenceNumber")]
    pub item_sequence_number: String,
    /// Buyer's Standard Identification Number (ASIN) of an item.
    #[serde(rename = "buyerProductIdentifier", skip_serializing_if = "Option::is_none")]
    pub buyer_product_identifier: Option<String>,
    /// The vendor selected product identification of the item.
    #[serde(rename = "vendorProductIdentifier", skip_serializing_if = "Option::is_none")]
    pub vendor_product_identifier: Option<String>,
    #[serde(rename = "netCost", skip_serializing_if = "Option::is_none")]
    pub net_cost: Option<Box<crate::models::Money>>,
    #[serde(rename = "listPrice", skip_serializing_if = "Option::is_none")]
    pub list_price: Option<Box<crate::models::Money>>,
    #[serde(rename = "orderedQuantity", skip_serializing_if = "Option::is_none")]
    pub ordered_quantity: Option<Box<crate::models::OrderItemStatusOrderedQuantity>>,
    #[serde(rename = "acknowledgementStatus", skip_serializing_if = "Option::is_none")]
    pub acknowledgement_status: Option<Box<crate::models::OrderItemStatusAcknowledgementStatus>>,
    #[serde(rename = "receivingStatus", skip_serializing_if = "Option::is_none")]
    pub receiving_status: Option<Box<crate::models::OrderItemStatusReceivingStatus>>,
}

impl OrderItemStatus {
    pub fn new(item_sequence_number: String) -> OrderItemStatus {
        OrderItemStatus {
            item_sequence_number,
            buyer_product_identifier: None,
            vendor_product_identifier: None,
            net_cost: None,
            list_price: None,
            ordered_quantity: None,
            acknowledgement_status: None,
            receiving_status: None,
        }
    }
}


