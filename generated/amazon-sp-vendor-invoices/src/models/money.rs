/*
 * Selling Partner API for Retail Procurement Payments
 *
 * The Selling Partner API for Retail Procurement Payments provides programmatic access to vendors payments data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Money : An amount of money, including units in the form of currency.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Money {
    /// Three-digit currency code in ISO 4217 format.
    #[serde(default, rename = "currencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// A decimal number with no loss of precision. Useful when precision loss is unacceptable, as with currencies. Follows RFC7159 for number representation. <br>**Pattern** : `^-?(0|([1-9]\\d*))(\\.\\d+)?([eE][+-]?\\d+)?$`.
    #[serde(default, rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl Money {
    /// An amount of money, including units in the form of currency.
    pub fn new() -> Money {
        Money {
            currency_code: None,
            amount: None,
        }
    }
}


