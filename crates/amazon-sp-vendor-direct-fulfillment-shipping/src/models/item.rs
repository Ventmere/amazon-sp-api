/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: 2021-12-28
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Item : Details of the item being shipped.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Item {
    /// Item Sequence Number for the item. This must be the same value as sent in order for a given item.
    #[serde(rename = "itemSequenceNumber")]
    pub item_sequence_number: i32,
    /// Buyer's Standard Identification Number (ASIN) of an item. Either buyerProductIdentifier or vendorProductIdentifier is required.
    #[serde(rename = "buyerProductIdentifier", skip_serializing_if = "Option::is_none")]
    pub buyer_product_identifier: Option<String>,
    /// The vendor selected product identification of the item. Should be the same as was sent in the purchase order, like SKU Number.
    #[serde(rename = "vendorProductIdentifier", skip_serializing_if = "Option::is_none")]
    pub vendor_product_identifier: Option<String>,
    #[serde(rename = "shippedQuantity")]
    pub shipped_quantity: Box<crate::models::ItemQuantity>,
}

impl Item {
    /// Details of the item being shipped.
    pub fn new(item_sequence_number: i32, shipped_quantity: crate::models::ItemQuantity) -> Item {
        Item {
            item_sequence_number,
            buyer_product_identifier: None,
            vendor_product_identifier: None,
            shipped_quantity: Box::new(shipped_quantity),
        }
    }
}


