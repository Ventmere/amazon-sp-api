/*
 * Selling Partner API for Direct Fulfillment Payments
 *
 * The Selling Partner API for Direct Fulfillment Payments provides programmatic access to a direct fulfillment vendor's invoice data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SubmitInvoiceResponse : The response schema for the submitInvoice operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitInvoiceResponse {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::TransactionReference>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl SubmitInvoiceResponse {
    /// The response schema for the submitInvoice operation.
    pub fn new() -> SubmitInvoiceResponse {
        SubmitInvoiceResponse {
            payload: None,
            errors: None,
        }
    }
}


