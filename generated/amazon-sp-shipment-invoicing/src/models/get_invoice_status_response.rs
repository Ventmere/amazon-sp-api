/*
 * Selling Partner API for Shipment Invoicing
 *
 * The Selling Partner API for Shipment Invoicing helps you programmatically retrieve shipment invoice information in the Brazil marketplace for a selling partner’s Fulfillment by Amazon (FBA) orders.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetInvoiceStatusResponse : The response schema for the getInvoiceStatus operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetInvoiceStatusResponse {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::ShipmentInvoiceStatusResponse>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetInvoiceStatusResponse {
    /// The response schema for the getInvoiceStatus operation.
    pub fn new() -> GetInvoiceStatusResponse {
        GetInvoiceStatusResponse {
            payload: None,
            errors: None,
        }
    }
}

