/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateFulfillmentOrderItem : Item information for updating a fulfillment order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateFulfillmentOrderItem {
    /// The seller SKU of the item.
    #[serde(rename = "sellerSku", skip_serializing_if = "Option::is_none")]
    pub seller_sku: Option<String>,
    /// Identifies the fulfillment order item to update. Created with a previous call to the createFulfillmentOrder operation.
    #[serde(rename = "sellerFulfillmentOrderItemId")]
    pub seller_fulfillment_order_item_id: String,
    /// The item quantity.
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// A message to the gift recipient, if applicable.
    #[serde(rename = "giftMessage", skip_serializing_if = "Option::is_none")]
    pub gift_message: Option<String>,
    /// Item-specific text that displays in recipient-facing materials such as the outbound shipment packing slip.
    #[serde(rename = "displayableComment", skip_serializing_if = "Option::is_none")]
    pub displayable_comment: Option<String>,
    /// Amazon's fulfillment network SKU of the item.
    #[serde(rename = "fulfillmentNetworkSku", skip_serializing_if = "Option::is_none")]
    pub fulfillment_network_sku: Option<String>,
    /// Indicates whether the item is sellable or unsellable.
    #[serde(rename = "orderItemDisposition", skip_serializing_if = "Option::is_none")]
    pub order_item_disposition: Option<String>,
    #[serde(rename = "perUnitDeclaredValue", skip_serializing_if = "Option::is_none")]
    pub per_unit_declared_value: Option<Box<crate::models::Money>>,
    #[serde(rename = "perUnitPrice", skip_serializing_if = "Option::is_none")]
    pub per_unit_price: Option<Box<crate::models::Money>>,
    #[serde(rename = "perUnitTax", skip_serializing_if = "Option::is_none")]
    pub per_unit_tax: Option<Box<crate::models::Money>>,
}

impl UpdateFulfillmentOrderItem {
    /// Item information for updating a fulfillment order.
    pub fn new(seller_fulfillment_order_item_id: String, quantity: i32) -> UpdateFulfillmentOrderItem {
        UpdateFulfillmentOrderItem {
            seller_sku: None,
            seller_fulfillment_order_item_id,
            quantity,
            gift_message: None,
            displayable_comment: None,
            fulfillment_network_sku: None,
            order_item_disposition: None,
            per_unit_declared_value: None,
            per_unit_price: None,
            per_unit_tax: None,
        }
    }
}

