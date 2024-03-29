/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Promotion : A promotion applied to an item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Promotion {
    /// The type of promotion.
    #[serde(default, rename = "PromotionType", skip_serializing_if = "Option::is_none")]
    pub promotion_type: Option<String>,
    /// The seller-specified identifier for the promotion.
    #[serde(default, rename = "PromotionId", skip_serializing_if = "Option::is_none")]
    pub promotion_id: Option<String>,
    #[serde(default, rename = "PromotionAmount", skip_serializing_if = "Option::is_none")]
    pub promotion_amount: Option<Box<crate::models::Currency>>,
}

impl Promotion {
    /// A promotion applied to an item.
    pub fn new() -> Promotion {
        Promotion {
            promotion_type: None,
            promotion_id: None,
            promotion_amount: None,
        }
    }
}


