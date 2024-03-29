/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FulfillmentShipmentItem : Item information for a shipment in a fulfillment order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FulfillmentShipmentItem {
    /// The seller SKU of the item.
    #[serde(default, rename = "sellerSku")]
    pub seller_sku: String,
    /// The fulfillment order item identifier that the seller created and submitted with a call to the createFulfillmentOrder operation.
    #[serde(default, rename = "sellerFulfillmentOrderItemId")]
    pub seller_fulfillment_order_item_id: String,
    /// The item quantity.
    #[serde(default, rename = "quantity")]
    pub quantity: i32,
    /// An identifier for the package that contains the item quantity.
    #[serde(default, rename = "packageNumber", skip_serializing_if = "Option::is_none")]
    pub package_number: Option<i32>,
    /// The serial number of the shipped item.
    #[serde(default, rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}

impl FulfillmentShipmentItem {
    /// Item information for a shipment in a fulfillment order.
    pub fn new(seller_sku: String, seller_fulfillment_order_item_id: String, quantity: i32) -> FulfillmentShipmentItem {
        FulfillmentShipmentItem {
            seller_sku,
            seller_fulfillment_order_item_id,
            quantity,
            package_number: None,
            serial_number: None,
        }
    }
}


