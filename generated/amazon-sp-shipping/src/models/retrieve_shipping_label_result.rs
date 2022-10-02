/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RetrieveShippingLabelResult : The payload schema for the retrieveShippingLabel operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RetrieveShippingLabelResult {
    /// Contains binary image data encoded as a base-64 string.
    #[serde(rename = "labelStream")]
    pub label_stream: String,
    #[serde(rename = "labelSpecification")]
    pub label_specification: Box<crate::models::LabelSpecification>,
}

impl RetrieveShippingLabelResult {
    /// The payload schema for the retrieveShippingLabel operation.
    pub fn new(label_stream: String, label_specification: crate::models::LabelSpecification) -> RetrieveShippingLabelResult {
        RetrieveShippingLabelResult {
            label_stream,
            label_specification: Box::new(label_specification),
        }
    }
}


