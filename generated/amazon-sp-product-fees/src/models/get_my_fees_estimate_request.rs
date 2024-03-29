/*
 * Selling Partner API for Product Fees
 *
 * The Selling Partner API for Product Fees lets you programmatically retrieve estimated fees for a product. You can then account for those fees in your pricing.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetMyFeesEstimateRequest : Request schema.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMyFeesEstimateRequest {
    #[serde(default, rename = "FeesEstimateRequest", skip_serializing_if = "Option::is_none")]
    pub fees_estimate_request: Option<Box<crate::models::FeesEstimateRequest>>,
}

impl GetMyFeesEstimateRequest {
    /// Request schema.
    pub fn new() -> GetMyFeesEstimateRequest {
        GetMyFeesEstimateRequest {
            fees_estimate_request: None,
        }
    }
}


