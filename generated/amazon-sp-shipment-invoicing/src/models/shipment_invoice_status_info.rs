/*
 * Selling Partner API for Shipment Invoicing
 *
 * The Selling Partner API for Shipment Invoicing helps you programmatically retrieve shipment invoice information in the Brazil marketplace for a selling partner’s Fulfillment by Amazon (FBA) orders.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentInvoiceStatusInfo : The shipment invoice status information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipmentInvoiceStatusInfo {
    /// The Amazon-defined shipment identifier.
    #[serde(rename = "AmazonShipmentId", skip_serializing_if = "Option::is_none")]
    pub amazon_shipment_id: Option<String>,
    #[serde(rename = "InvoiceStatus", skip_serializing_if = "Option::is_none")]
    pub invoice_status: Option<crate::models::ShipmentInvoiceStatus>,
}

impl ShipmentInvoiceStatusInfo {
    /// The shipment invoice status information.
    pub fn new() -> ShipmentInvoiceStatusInfo {
        ShipmentInvoiceStatusInfo {
            amazon_shipment_id: None,
            invoice_status: None,
        }
    }
}

