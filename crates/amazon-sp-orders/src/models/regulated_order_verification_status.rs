/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RegulatedOrderVerificationStatus : The verification status of the order along with associated approval or rejection metadata.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegulatedOrderVerificationStatus {
    #[serde(rename = "Status")]
    pub status: crate::models::VerificationStatus,
    /// Whether the regulated information provided in the order requires a review by the merchant.
    #[serde(rename = "RequiresMerchantAction")]
    pub requires_merchant_action: bool,
    /// A list of valid rejection reasons that may be used to reject the order's regulated information.
    #[serde(rename = "ValidRejectionReasons")]
    pub valid_rejection_reasons: Vec<crate::models::RejectionReason>,
    #[serde(rename = "RejectionReason", skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<Box<crate::models::RejectionReason>>,
    /// The date the order was reviewed. In ISO 8601 date time format.
    #[serde(rename = "ReviewDate", skip_serializing_if = "Option::is_none")]
    pub review_date: Option<String>,
    /// The identifier for the order's regulated information reviewer.
    #[serde(rename = "ExternalReviewerId", skip_serializing_if = "Option::is_none")]
    pub external_reviewer_id: Option<String>,
}

impl RegulatedOrderVerificationStatus {
    /// The verification status of the order along with associated approval or rejection metadata.
    pub fn new(status: crate::models::VerificationStatus, requires_merchant_action: bool, valid_rejection_reasons: Vec<crate::models::RejectionReason>) -> RegulatedOrderVerificationStatus {
        RegulatedOrderVerificationStatus {
            status,
            requires_merchant_action,
            valid_rejection_reasons,
            rejection_reason: None,
            review_date: None,
            external_reviewer_id: None,
        }
    }
}


