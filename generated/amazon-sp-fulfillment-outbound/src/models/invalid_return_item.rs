/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvalidReturnItem : An item that is invalid for return.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvalidReturnItem {
    /// An identifier assigned by the seller to the return item.
    #[serde(rename = "sellerReturnItemId")]
    pub seller_return_item_id: String,
    /// The identifier assigned to the item by the seller when the fulfillment order was created.
    #[serde(rename = "sellerFulfillmentOrderItemId")]
    pub seller_fulfillment_order_item_id: String,
    #[serde(rename = "invalidItemReason")]
    pub invalid_item_reason: Box<crate::models::InvalidItemReason>,
}

impl InvalidReturnItem {
    /// An item that is invalid for return.
    pub fn new(seller_return_item_id: String, seller_fulfillment_order_item_id: String, invalid_item_reason: crate::models::InvalidItemReason) -> InvalidReturnItem {
        InvalidReturnItem {
            seller_return_item_id,
            seller_fulfillment_order_item_id,
            invalid_item_reason: Box::new(invalid_item_reason),
        }
    }
}

