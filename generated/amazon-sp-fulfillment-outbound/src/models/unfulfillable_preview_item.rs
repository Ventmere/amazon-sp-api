/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UnfulfillablePreviewItem : Information about unfulfillable items in a fulfillment order preview.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UnfulfillablePreviewItem {
    /// The seller SKU of the item.
    #[serde(default, rename = "sellerSku")]
    pub seller_sku: String,
    /// The item quantity.
    #[serde(default, rename = "quantity")]
    pub quantity: i32,
    /// A fulfillment order item identifier created with a call to the getFulfillmentPreview operation.
    #[serde(default, rename = "sellerFulfillmentOrderItemId")]
    pub seller_fulfillment_order_item_id: String,
    #[serde(default, rename = "itemUnfulfillableReasons", skip_serializing_if = "Option::is_none")]
    pub item_unfulfillable_reasons: Option<Vec<String>>,
}

impl UnfulfillablePreviewItem {
    /// Information about unfulfillable items in a fulfillment order preview.
    pub fn new(seller_sku: String, quantity: i32, seller_fulfillment_order_item_id: String) -> UnfulfillablePreviewItem {
        UnfulfillablePreviewItem {
            seller_sku,
            quantity,
            seller_fulfillment_order_item_id,
            item_unfulfillable_reasons: None,
        }
    }
}


