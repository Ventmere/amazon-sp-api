/*
 * Selling Partner API for Retail Procurement Payments
 *
 * The Selling Partner API for Retail Procurement Payments provides programmatic access to vendors payments data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SubmitInvoicesRequest : The request schema for the submitInvoices operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitInvoicesRequest {
    #[serde(rename = "invoices", skip_serializing_if = "Option::is_none")]
    pub invoices: Option<Vec<crate::models::Invoice>>,
}

impl SubmitInvoicesRequest {
    /// The request schema for the submitInvoices operation.
    pub fn new() -> SubmitInvoicesRequest {
        SubmitInvoicesRequest {
            invoices: None,
        }
    }
}


