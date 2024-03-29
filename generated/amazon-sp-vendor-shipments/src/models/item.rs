/*
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Item : Details of the item being shipped.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Item {
    /// Item sequence number for the item. The first item will be 001, the second 002, and so on. This number is used as a reference to refer to this item from the carton or pallet level.
    #[serde(default, rename = "itemSequenceNumber")]
    pub item_sequence_number: String,
    /// Amazon Standard Identification Number (ASIN) of an item.
    #[serde(default, rename = "amazonProductIdentifier", skip_serializing_if = "Option::is_none")]
    pub amazon_product_identifier: Option<String>,
    /// The vendor selected product identification of the item. Should be the same as was sent in the purchase order.
    #[serde(default, rename = "vendorProductIdentifier", skip_serializing_if = "Option::is_none")]
    pub vendor_product_identifier: Option<String>,
    #[serde(default, rename = "shippedQuantity")]
    pub shipped_quantity: Box<crate::models::ItemQuantity>,
    #[serde(default, rename = "itemDetails", skip_serializing_if = "Option::is_none")]
    pub item_details: Option<Box<crate::models::ItemDetails>>,
}

impl Item {
    /// Details of the item being shipped.
    pub fn new(item_sequence_number: String, shipped_quantity: crate::models::ItemQuantity) -> Item {
        Item {
            item_sequence_number,
            amazon_product_identifier: None,
            vendor_product_identifier: None,
            shipped_quantity: Box::new(shipped_quantity),
            item_details: None,
        }
    }
}


