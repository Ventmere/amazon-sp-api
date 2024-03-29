/*
 * Selling Partner API for FBA Small And Light
 *
 * The Selling Partner API for FBA Small and Light lets you help sellers manage their listings in the Small and Light program. The program reduces the cost of fulfilling orders for small and lightweight FBA inventory. You can enroll or remove items from the program and check item eligibility and enrollment status. You can also preview the estimated program fees charged to a seller for items sold while enrolled in the program.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SmallAndLightFeePreviewRequest : Request schema for submitting items for which to retrieve fee estimates.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SmallAndLightFeePreviewRequest {
    /// A marketplace identifier.
    #[serde(default, rename = "marketplaceId")]
    pub marketplace_id: String,
    /// A list of items for which to retrieve fee estimates (limit: 25).
    #[serde(default, rename = "items")]
    pub items: Vec<crate::models::Item>,
}

impl SmallAndLightFeePreviewRequest {
    /// Request schema for submitting items for which to retrieve fee estimates.
    pub fn new(marketplace_id: String, items: Vec<crate::models::Item>) -> SmallAndLightFeePreviewRequest {
        SmallAndLightFeePreviewRequest {
            marketplace_id,
            items,
        }
    }
}


