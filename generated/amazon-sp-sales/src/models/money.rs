/*
 * Selling Partner API for Sales
 *
 * The Selling Partner API for Sales provides APIs related to sales performance.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Money : The currency type and the amount.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Money {
    /// Three-digit currency code. In ISO 4217 format.
    #[serde(default, rename = "currencyCode")]
    pub currency_code: String,
    /// A decimal number with no loss of precision. Useful when precision loss is unnaceptable, as with currencies. Follows RFC7159 for number representation.
    #[serde(default, rename = "amount")]
    pub amount: String,
}

impl Money {
    /// The currency type and the amount.
    pub fn new(currency_code: String, amount: String) -> Money {
        Money {
            currency_code,
            amount,
        }
    }
}


