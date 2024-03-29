/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCustomerInvoiceResponse : The response schema for the getCustomerInvoice operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCustomerInvoiceResponse {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::CustomerInvoice>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetCustomerInvoiceResponse {
    /// The response schema for the getCustomerInvoice operation.
    pub fn new() -> GetCustomerInvoiceResponse {
        GetCustomerInvoiceResponse {
            payload: None,
            errors: None,
        }
    }
}


