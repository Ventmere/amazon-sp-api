/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentDetails : Details about a shipment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipmentDetails {
    /// This field indicates the date of the departure of the shipment from vendor's location. Vendors are requested to send ASNs within 30 minutes of departure from their warehouse/distribution center or at least 6 hours prior to the appointment time at the Amazon destination warehouse, whichever is sooner. Shipped date mentioned in the Shipment Confirmation should not be in the future.
    #[serde(default, rename = "shippedDate")]
    pub shipped_date: String,
    /// Indicate the shipment status.
    #[serde(default, rename = "shipmentStatus")]
    pub shipment_status: ShipmentStatus,
    /// Provide the priority of the shipment.
    #[serde(default, rename = "isPriorityShipment", skip_serializing_if = "Option::is_none")]
    pub is_priority_shipment: Option<bool>,
    /// The vendor order number is a unique identifier generated by a vendor for their reference.
    #[serde(default, rename = "vendorOrderNumber", skip_serializing_if = "Option::is_none")]
    pub vendor_order_number: Option<String>,
    /// Date on which the shipment is expected to reach the buyer's warehouse. It needs to be an estimate based on the average transit time between the ship-from location and the destination. The exact appointment time will be provided by buyer and is potentially not known when creating the shipment confirmation.
    #[serde(default, rename = "estimatedDeliveryDate", skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_date: Option<String>,
}

impl ShipmentDetails {
    /// Details about a shipment.
    pub fn new(shipped_date: String, shipment_status: ShipmentStatus) -> ShipmentDetails {
        ShipmentDetails {
            shipped_date,
            shipment_status,
            is_priority_shipment: None,
            vendor_order_number: None,
            estimated_delivery_date: None,
        }
    }
}

/// Indicate the shipment status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShipmentStatus {
    #[serde(rename = "SHIPPED")]
    SHIPPED,
    #[serde(rename = "FLOOR_DENIAL")]
    FLOORDENIAL,
}

impl Default for ShipmentStatus {
    fn default() -> ShipmentStatus {
        Self::SHIPPED
    }
}

