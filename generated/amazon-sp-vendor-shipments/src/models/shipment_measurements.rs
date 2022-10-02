/*
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentMeasurements : Shipment measurement details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipmentMeasurements {
    #[serde(rename = "grossShipmentWeight", skip_serializing_if = "Option::is_none")]
    pub gross_shipment_weight: Option<Box<crate::models::Weight>>,
    #[serde(rename = "shipmentVolume", skip_serializing_if = "Option::is_none")]
    pub shipment_volume: Option<Box<crate::models::Volume>>,
    /// Number of cartons present in the shipment. Provide the cartonCount only for unpalletized shipments.
    #[serde(rename = "cartonCount", skip_serializing_if = "Option::is_none")]
    pub carton_count: Option<i32>,
    /// Number of pallets present in the shipment. Provide the palletCount only for palletized shipments.
    #[serde(rename = "palletCount", skip_serializing_if = "Option::is_none")]
    pub pallet_count: Option<i32>,
}

impl ShipmentMeasurements {
    /// Shipment measurement details.
    pub fn new() -> ShipmentMeasurements {
        ShipmentMeasurements {
            gross_shipment_weight: None,
            shipment_volume: None,
            carton_count: None,
            pallet_count: None,
        }
    }
}

