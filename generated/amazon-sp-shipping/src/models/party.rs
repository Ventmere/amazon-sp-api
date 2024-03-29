/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Party : The account related with the shipment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Party {
    /// This is the Amazon Shipping account id generated during the Amazon Shipping onboarding process.
    #[serde(default, rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl Party {
    /// The account related with the shipment.
    pub fn new() -> Party {
        Party {
            account_id: None,
        }
    }
}


