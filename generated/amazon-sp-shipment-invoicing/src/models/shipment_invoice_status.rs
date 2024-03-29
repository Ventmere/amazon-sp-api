/*
 * Selling Partner API for Shipment Invoicing
 *
 * The Selling Partner API for Shipment Invoicing helps you programmatically retrieve shipment invoice information in the Brazil marketplace for a selling partner’s Fulfillment by Amazon (FBA) orders.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentInvoiceStatus : The shipment invoice status.

/// The shipment invoice status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShipmentInvoiceStatus {
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Errored")]
    Errored,
    #[serde(rename = "NotFound")]
    NotFound,

}

impl ToString for ShipmentInvoiceStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Processing => String::from("Processing"),
            Self::Accepted => String::from("Accepted"),
            Self::Errored => String::from("Errored"),
            Self::NotFound => String::from("NotFound"),
        }
    }
}

impl Default for ShipmentInvoiceStatus {
    fn default() -> ShipmentInvoiceStatus {
        Self::Processing
    }
}




