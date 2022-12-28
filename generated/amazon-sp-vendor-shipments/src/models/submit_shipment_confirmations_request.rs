/*
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SubmitShipmentConfirmationsRequest : The request schema for the SubmitShipmentConfirmations operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitShipmentConfirmationsRequest {
    #[serde(rename = "shipmentConfirmations", skip_serializing_if = "Option::is_none")]
    pub shipment_confirmations: Option<Vec<crate::models::ShipmentConfirmation>>,
}

impl SubmitShipmentConfirmationsRequest {
    /// The request schema for the SubmitShipmentConfirmations operation.
    pub fn new() -> SubmitShipmentConfirmationsRequest {
        SubmitShipmentConfirmationsRequest {
            shipment_confirmations: None,
        }
    }
}

