/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TaxWithholdingPeriod : Period which taxwithholding on seller's account is calculated.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxWithholdingPeriod {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl TaxWithholdingPeriod {
    /// Period which taxwithholding on seller's account is calculated.
    pub fn new() -> TaxWithholdingPeriod {
        TaxWithholdingPeriod {
            start_date: None,
            end_date: None,
        }
    }
}


