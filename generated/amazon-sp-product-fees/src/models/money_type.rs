/*
 * Selling Partner API for Product Fees
 *
 * The Selling Partner API for Product Fees lets you programmatically retrieve estimated fees for a product. You can then account for those fees in your pricing.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MoneyType {
    /// The currency code in ISO 4217 format.
    #[serde(rename = "CurrencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// The monetary value.
    #[serde(rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
}

impl MoneyType {
    pub fn new() -> MoneyType {
        MoneyType {
            currency_code: None,
            amount: None,
        }
    }
}


