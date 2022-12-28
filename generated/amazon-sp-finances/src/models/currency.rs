/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Currency : A currency type and amount.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Currency {
    /// The three-digit currency code in ISO 4217 format.
    #[serde(rename = "CurrencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "CurrencyAmount", skip_serializing_if = "Option::is_none")]
    pub currency_amount: Option<f32>,
}

impl Currency {
    /// A currency type and amount.
    pub fn new() -> Currency {
        Currency {
            currency_code: None,
            currency_amount: None,
        }
    }
}

