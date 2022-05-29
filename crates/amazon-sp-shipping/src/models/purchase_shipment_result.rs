/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PurchaseShipmentResult : The payload schema for the purchaseShipment operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PurchaseShipmentResult {
    /// The unique shipment identifier.
    #[serde(rename = "shipmentId")]
    pub shipment_id: String,
    #[serde(rename = "serviceRate")]
    pub service_rate: Box<crate::models::ServiceRate>,
    /// A list of label results
    #[serde(rename = "labelResults")]
    pub label_results: Vec<crate::models::LabelResult>,
}

impl PurchaseShipmentResult {
    /// The payload schema for the purchaseShipment operation.
    pub fn new(shipment_id: String, service_rate: crate::models::ServiceRate, label_results: Vec<crate::models::LabelResult>) -> PurchaseShipmentResult {
        PurchaseShipmentResult {
            shipment_id,
            service_rate: Box::new(service_rate),
            label_results,
        }
    }
}


