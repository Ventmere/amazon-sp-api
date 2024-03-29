/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitShippingLabelsRequest {
    #[serde(default, rename = "shippingLabelRequests", skip_serializing_if = "Option::is_none")]
    pub shipping_label_requests: Option<Vec<crate::models::ShippingLabelRequest>>,
}

impl SubmitShippingLabelsRequest {
    pub fn new() -> SubmitShippingLabelsRequest {
        SubmitShippingLabelsRequest {
            shipping_label_requests: None,
        }
    }
}


