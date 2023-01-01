/*
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderAcknowledgementItem : Details of the item being acknowledged.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderAcknowledgementItem {
    /// Line item sequence number for the item.
    #[serde(default, rename = "itemSequenceNumber", skip_serializing_if = "Option::is_none")]
    pub item_sequence_number: Option<String>,
    /// Amazon Standard Identification Number (ASIN) of an item.
    #[serde(default, rename = "amazonProductIdentifier", skip_serializing_if = "Option::is_none")]
    pub amazon_product_identifier: Option<String>,
    /// The vendor selected product identification of the item. Should be the same as was sent in the purchase order.
    #[serde(default, rename = "vendorProductIdentifier", skip_serializing_if = "Option::is_none")]
    pub vendor_product_identifier: Option<String>,
    #[serde(default, rename = "orderedQuantity")]
    pub ordered_quantity: Box<crate::models::ItemQuantity>,
    #[serde(default, rename = "netCost", skip_serializing_if = "Option::is_none")]
    pub net_cost: Option<Box<crate::models::Money>>,
    #[serde(default, rename = "listPrice", skip_serializing_if = "Option::is_none")]
    pub list_price: Option<Box<crate::models::Money>>,
    /// The discount multiplier that should be applied to the price if a vendor sells books with a list price. This is a multiplier factor to arrive at a final discounted price. A multiplier of .90 would be the factor if a 10% discount is given.
    #[serde(default, rename = "discountMultiplier", skip_serializing_if = "Option::is_none")]
    pub discount_multiplier: Option<String>,
    /// This is used to indicate acknowledged quantity.
    #[serde(default, rename = "itemAcknowledgements")]
    pub item_acknowledgements: Vec<crate::models::OrderItemAcknowledgement>,
}

impl OrderAcknowledgementItem {
    /// Details of the item being acknowledged.
    pub fn new(ordered_quantity: crate::models::ItemQuantity, item_acknowledgements: Vec<crate::models::OrderItemAcknowledgement>) -> OrderAcknowledgementItem {
        OrderAcknowledgementItem {
            item_sequence_number: None,
            amazon_product_identifier: None,
            vendor_product_identifier: None,
            ordered_quantity: Box::new(ordered_quantity),
            net_cost: None,
            list_price: None,
            discount_multiplier: None,
            item_acknowledgements,
        }
    }
}


