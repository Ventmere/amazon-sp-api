/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SafetReimbursementEvent : A SAFE-T claim reimbursement on the seller's account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SafetReimbursementEvent {
    #[serde(default, rename = "PostedDate", skip_serializing_if = "Option::is_none")]
    pub posted_date: Option<String>,
    /// A SAFE-T claim identifier.
    #[serde(default, rename = "SAFETClaimId", skip_serializing_if = "Option::is_none")]
    pub safet_claim_id: Option<String>,
    #[serde(default, rename = "ReimbursedAmount", skip_serializing_if = "Option::is_none")]
    pub reimbursed_amount: Option<Box<crate::models::Currency>>,
    /// Indicates why the seller was reimbursed.
    #[serde(default, rename = "ReasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// A list of SAFETReimbursementItems.
    #[serde(default, rename = "SAFETReimbursementItemList", skip_serializing_if = "Option::is_none")]
    pub safet_reimbursement_item_list: Option<Vec<crate::models::SafetReimbursementItem>>,
}

impl SafetReimbursementEvent {
    /// A SAFE-T claim reimbursement on the seller's account.
    pub fn new() -> SafetReimbursementEvent {
        SafetReimbursementEvent {
            posted_date: None,
            safet_claim_id: None,
            reimbursed_amount: None,
            reason_code: None,
            safet_reimbursement_item_list: None,
        }
    }
}


