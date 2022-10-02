/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FulfillmentShipment : Delivery and item information for a shipment in a fulfillment order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FulfillmentShipment {
    /// A shipment identifier assigned by Amazon.
    #[serde(rename = "amazonShipmentId")]
    pub amazon_shipment_id: String,
    /// An identifier for the fulfillment center that the shipment will be sent from.
    #[serde(rename = "fulfillmentCenterId")]
    pub fulfillment_center_id: String,
    /// The current status of the shipment.
    #[serde(rename = "fulfillmentShipmentStatus")]
    pub fulfillment_shipment_status: FulfillmentShipmentStatus,
    #[serde(rename = "shippingDate", skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<String>,
    #[serde(rename = "estimatedArrivalDate", skip_serializing_if = "Option::is_none")]
    pub estimated_arrival_date: Option<String>,
    /// Provides additional insight into shipment timeline. Primairly used to communicate that actual delivery dates aren't available.
    #[serde(rename = "shippingNotes", skip_serializing_if = "Option::is_none")]
    pub shipping_notes: Option<Vec<String>>,
    /// An array of fulfillment shipment item information.
    #[serde(rename = "fulfillmentShipmentItem")]
    pub fulfillment_shipment_item: Vec<crate::models::FulfillmentShipmentItem>,
    /// An array of fulfillment shipment package information.
    #[serde(rename = "fulfillmentShipmentPackage", skip_serializing_if = "Option::is_none")]
    pub fulfillment_shipment_package: Option<Vec<crate::models::FulfillmentShipmentPackage>>,
}

impl FulfillmentShipment {
    /// Delivery and item information for a shipment in a fulfillment order.
    pub fn new(amazon_shipment_id: String, fulfillment_center_id: String, fulfillment_shipment_status: FulfillmentShipmentStatus, fulfillment_shipment_item: Vec<crate::models::FulfillmentShipmentItem>) -> FulfillmentShipment {
        FulfillmentShipment {
            amazon_shipment_id,
            fulfillment_center_id,
            fulfillment_shipment_status,
            shipping_date: None,
            estimated_arrival_date: None,
            shipping_notes: None,
            fulfillment_shipment_item,
            fulfillment_shipment_package: None,
        }
    }
}

/// The current status of the shipment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FulfillmentShipmentStatus {
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "SHIPPED")]
    SHIPPED,
    #[serde(rename = "CANCELLED_BY_FULFILLER")]
    CANCELLEDBYFULFILLER,
    #[serde(rename = "CANCELLED_BY_SELLER")]
    CANCELLEDBYSELLER,
}

impl Default for FulfillmentShipmentStatus {
    fn default() -> FulfillmentShipmentStatus {
        Self::PENDING
    }
}

