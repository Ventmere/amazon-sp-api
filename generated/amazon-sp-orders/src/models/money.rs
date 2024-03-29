/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Money : The monetary value of the order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Money {
    /// The three-digit currency code. In ISO 4217 format.
    #[serde(default, rename = "CurrencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// The currency amount.
    #[serde(default, rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl Money {
    /// The monetary value of the order.
    pub fn new() -> Money {
        Money {
            currency_code: None,
            amount: None,
        }
    }
}


