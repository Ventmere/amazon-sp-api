/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Label : The label details of the container.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Label {
    /// Contains binary image data encoded as a base-64 string.
    #[serde(rename = "labelStream", skip_serializing_if = "Option::is_none")]
    pub label_stream: Option<String>,
    #[serde(rename = "labelSpecification", skip_serializing_if = "Option::is_none")]
    pub label_specification: Option<Box<crate::models::LabelSpecification>>,
}

impl Label {
    /// The label details of the container.
    pub fn new() -> Label {
        Label {
            label_stream: None,
            label_specification: None,
        }
    }
}


