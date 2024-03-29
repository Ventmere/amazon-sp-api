/*
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentDetails : Shipment details required for the shipment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipmentDetails {
    /// When true, this is a priority shipment.
    #[serde(default, rename = "isPriorityShipment")]
    pub is_priority_shipment: bool,
    /// When true, this order is part of a scheduled delivery program.
    #[serde(default, rename = "isScheduledDeliveryShipment", skip_serializing_if = "Option::is_none")]
    pub is_scheduled_delivery_shipment: Option<bool>,
    /// When true, a packing slip is required to be sent to the customer.
    #[serde(default, rename = "isPslipRequired")]
    pub is_pslip_required: bool,
    /// When true, the order contain a gift. Include the gift message and gift wrap information.
    #[serde(default, rename = "isGift", skip_serializing_if = "Option::is_none")]
    pub is_gift: Option<bool>,
    /// Ship method to be used for shipping the order. Amazon defines ship method codes indicating the shipping carrier and shipment service level. To see the full list of ship methods in use, including both the code and the friendly name, search the 'Help' section on Vendor Central for 'ship methods'.
    #[serde(default, rename = "shipMethod")]
    pub ship_method: String,
    #[serde(default, rename = "shipmentDates")]
    pub shipment_dates: Box<crate::models::ShipmentDates>,
    /// Message to customer for order status.
    #[serde(default, rename = "messageToCustomer")]
    pub message_to_customer: String,
}

impl ShipmentDetails {
    /// Shipment details required for the shipment.
    pub fn new(is_priority_shipment: bool, is_pslip_required: bool, ship_method: String, shipment_dates: crate::models::ShipmentDates, message_to_customer: String) -> ShipmentDetails {
        ShipmentDetails {
            is_priority_shipment,
            is_scheduled_delivery_shipment: None,
            is_pslip_required,
            is_gift: None,
            ship_method,
            shipment_dates: Box::new(shipment_dates),
            message_to_customer,
        }
    }
}


