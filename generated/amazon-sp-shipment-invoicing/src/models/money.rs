/*
 * Selling Partner API for Shipment Invoicing
 *
 * The Selling Partner API for Shipment Invoicing helps you programmatically retrieve shipment invoice information in the Brazil marketplace for a selling partner’s Fulfillment by Amazon (FBA) orders.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Money : The currency type and amount.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Money {
    /// Three-digit currency code in ISO 4217 format.
    #[serde(default, rename = "CurrencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// The currency amount.
    #[serde(default, rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl Money {
    /// The currency type and amount.
    pub fn new() -> Money {
        Money {
            currency_code: None,
            amount: None,
        }
    }
}


