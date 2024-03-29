/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Money : An amount of money, including units in the form of currency.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Money {
    /// Three digit currency code in ISO 4217 format.
    #[serde(default, rename = "currencyCode")]
    pub currency_code: String,
    /// A decimal number with no loss of precision. Useful when precision loss is unacceptable, as with currencies. Follows RFC7159 for number representation.
    #[serde(default, rename = "value")]
    pub value: String,
}

impl Money {
    /// An amount of money, including units in the form of currency.
    pub fn new(currency_code: String, value: String) -> Money {
        Money {
            currency_code,
            value,
        }
    }
}


