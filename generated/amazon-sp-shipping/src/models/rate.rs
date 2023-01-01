/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Rate : The available rate that can be used to send the shipment



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Rate {
    /// An identifier for the rate.
    #[serde(default, rename = "rateId", skip_serializing_if = "Option::is_none")]
    pub rate_id: Option<String>,
    #[serde(default, rename = "totalCharge", skip_serializing_if = "Option::is_none")]
    pub total_charge: Option<Box<crate::models::Currency>>,
    #[serde(default, rename = "billedWeight", skip_serializing_if = "Option::is_none")]
    pub billed_weight: Option<Box<crate::models::Weight>>,
    /// The time after which the offering will expire.
    #[serde(default, rename = "expirationTime", skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, rename = "serviceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<crate::models::ServiceType>,
    #[serde(default, rename = "promise", skip_serializing_if = "Option::is_none")]
    pub promise: Option<Box<crate::models::ShippingPromiseSet>>,
}

impl Rate {
    /// The available rate that can be used to send the shipment
    pub fn new() -> Rate {
        Rate {
            rate_id: None,
            total_charge: None,
            billed_weight: None,
            expiration_time: None,
            service_type: None,
            promise: None,
        }
    }
}


