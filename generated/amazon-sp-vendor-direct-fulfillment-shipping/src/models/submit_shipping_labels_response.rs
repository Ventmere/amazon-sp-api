/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SubmitShippingLabelsResponse : The response schema for the submitShippingLabelRequest operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitShippingLabelsResponse {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::TransactionReference>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl SubmitShippingLabelsResponse {
    /// The response schema for the submitShippingLabelRequest operation.
    pub fn new() -> SubmitShippingLabelsResponse {
        SubmitShippingLabelsResponse {
            payload: None,
            errors: None,
        }
    }
}


