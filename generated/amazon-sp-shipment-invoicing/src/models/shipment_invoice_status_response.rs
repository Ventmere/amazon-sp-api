/*
 * Selling Partner API for Shipment Invoicing
 *
 * The Selling Partner API for Shipment Invoicing helps you programmatically retrieve shipment invoice information in the Brazil marketplace for a selling partner’s Fulfillment by Amazon (FBA) orders.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentInvoiceStatusResponse : The shipment invoice status response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipmentInvoiceStatusResponse {
    #[serde(rename = "Shipments", skip_serializing_if = "Option::is_none")]
    pub shipments: Option<Box<crate::models::ShipmentInvoiceStatusInfo>>,
}

impl ShipmentInvoiceStatusResponse {
    /// The shipment invoice status response.
    pub fn new() -> ShipmentInvoiceStatusResponse {
        ShipmentInvoiceStatusResponse {
            shipments: None,
        }
    }
}

