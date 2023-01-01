/*
 * Selling Partner API for Product Fees
 *
 * The Selling Partner API for Product Fees lets you programmatically retrieve estimated fees for a product. You can then account for those fees in your pricing.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetMyFeesEstimateResult : Response schema.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMyFeesEstimateResult {
    #[serde(default, rename = "FeesEstimateResult", skip_serializing_if = "Option::is_none")]
    pub fees_estimate_result: Option<Box<crate::models::FeesEstimateResult>>,
}

impl GetMyFeesEstimateResult {
    /// Response schema.
    pub fn new() -> GetMyFeesEstimateResult {
        GetMyFeesEstimateResult {
            fees_estimate_result: None,
        }
    }
}


