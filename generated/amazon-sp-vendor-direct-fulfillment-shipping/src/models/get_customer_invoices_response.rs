/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCustomerInvoicesResponse : The response schema for the getCustomerInvoices operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCustomerInvoicesResponse {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::CustomerInvoiceList>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetCustomerInvoicesResponse {
    /// The response schema for the getCustomerInvoices operation.
    pub fn new() -> GetCustomerInvoicesResponse {
        GetCustomerInvoicesResponse {
            payload: None,
            errors: None,
        }
    }
}


