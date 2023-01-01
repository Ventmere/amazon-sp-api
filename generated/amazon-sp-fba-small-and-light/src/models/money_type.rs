/*
 * Selling Partner API for FBA Small And Light
 *
 * The Selling Partner API for FBA Small and Light lets you help sellers manage their listings in the Small and Light program. The program reduces the cost of fulfilling orders for small and lightweight FBA inventory. You can enroll or remove items from the program and check item eligibility and enrollment status. You can also preview the estimated program fees charged to a seller for items sold while enrolled in the program.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MoneyType {
    /// The currency code in ISO 4217 format.
    #[serde(default, rename = "currencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// The monetary value.
    #[serde(default, rename = "amount", skip_serializing_if = "Option::is_none")]
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


