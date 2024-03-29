/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReturnItem : An item that Amazon accepted for return.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReturnItem {
    /// An identifier assigned by the seller to the return item.
    #[serde(default, rename = "sellerReturnItemId")]
    pub seller_return_item_id: String,
    /// The identifier assigned to the item by the seller when the fulfillment order was created.
    #[serde(default, rename = "sellerFulfillmentOrderItemId")]
    pub seller_fulfillment_order_item_id: String,
    /// The identifier for the shipment that is associated with the return item.
    #[serde(default, rename = "amazonShipmentId")]
    pub amazon_shipment_id: String,
    /// The return reason code assigned to the return item by the seller.
    #[serde(default, rename = "sellerReturnReasonCode")]
    pub seller_return_reason_code: String,
    /// An optional comment about the return item.
    #[serde(default, rename = "returnComment", skip_serializing_if = "Option::is_none")]
    pub return_comment: Option<String>,
    /// The return reason code that the Amazon fulfillment center assigned to the return item.
    #[serde(default, rename = "amazonReturnReasonCode", skip_serializing_if = "Option::is_none")]
    pub amazon_return_reason_code: Option<String>,
    #[serde(default, rename = "status")]
    pub status: crate::models::FulfillmentReturnItemStatus,
    #[serde(default, rename = "statusChangedDate")]
    pub status_changed_date: String,
    /// Identifies the return authorization used to return this item. See ReturnAuthorization.
    #[serde(default, rename = "returnAuthorizationId", skip_serializing_if = "Option::is_none")]
    pub return_authorization_id: Option<String>,
    #[serde(default, rename = "returnReceivedCondition", skip_serializing_if = "Option::is_none")]
    pub return_received_condition: Option<crate::models::ReturnItemDisposition>,
    /// The identifier for the Amazon fulfillment center that processed the return item.
    #[serde(default, rename = "fulfillmentCenterId", skip_serializing_if = "Option::is_none")]
    pub fulfillment_center_id: Option<String>,
}

impl ReturnItem {
    /// An item that Amazon accepted for return.
    pub fn new(seller_return_item_id: String, seller_fulfillment_order_item_id: String, amazon_shipment_id: String, seller_return_reason_code: String, status: crate::models::FulfillmentReturnItemStatus, status_changed_date: String) -> ReturnItem {
        ReturnItem {
            seller_return_item_id,
            seller_fulfillment_order_item_id,
            amazon_shipment_id,
            seller_return_reason_code,
            return_comment: None,
            amazon_return_reason_code: None,
            status,
            status_changed_date,
            return_authorization_id: None,
            return_received_condition: None,
            fulfillment_center_id: None,
        }
    }
}


