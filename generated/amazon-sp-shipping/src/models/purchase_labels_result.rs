/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PurchaseLabelsResult : The payload schema for the purchaseLabels operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PurchaseLabelsResult {
    /// The unique shipment identifier.
    #[serde(rename = "shipmentId")]
    pub shipment_id: String,
    /// Client reference id.
    #[serde(rename = "clientReferenceId", skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    #[serde(rename = "acceptedRate")]
    pub accepted_rate: Box<crate::models::AcceptedRate>,
    /// A list of label results
    #[serde(rename = "labelResults")]
    pub label_results: Vec<crate::models::LabelResult>,
}

impl PurchaseLabelsResult {
    /// The payload schema for the purchaseLabels operation.
    pub fn new(shipment_id: String, accepted_rate: crate::models::AcceptedRate, label_results: Vec<crate::models::LabelResult>) -> PurchaseLabelsResult {
        PurchaseLabelsResult {
            shipment_id,
            client_reference_id: None,
            accepted_rate: Box::new(accepted_rate),
            label_results,
        }
    }
}

