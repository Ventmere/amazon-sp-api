/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AcceptedRate : The specific rate purchased for the shipment, or null if unpurchased.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AcceptedRate {
    #[serde(rename = "totalCharge", skip_serializing_if = "Option::is_none")]
    pub total_charge: Option<Box<crate::models::Currency>>,
    #[serde(rename = "billedWeight", skip_serializing_if = "Option::is_none")]
    pub billed_weight: Option<Box<crate::models::Weight>>,
    #[serde(rename = "serviceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<crate::models::ServiceType>,
    #[serde(rename = "promise", skip_serializing_if = "Option::is_none")]
    pub promise: Option<Box<crate::models::ShippingPromiseSet>>,
}

impl AcceptedRate {
    /// The specific rate purchased for the shipment, or null if unpurchased.
    pub fn new() -> AcceptedRate {
        AcceptedRate {
            total_charge: None,
            billed_weight: None,
            service_type: None,
            promise: None,
        }
    }
}

