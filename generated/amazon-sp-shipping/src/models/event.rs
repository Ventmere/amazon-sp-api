/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Event : An event of a shipment



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Event {
    /// The event code of a shipment, such as Departed, Received, and ReadyForReceive.
    #[serde(default, rename = "eventCode")]
    pub event_code: String,
    /// The date and time of an event for a shipment.
    #[serde(default, rename = "eventTime")]
    pub event_time: String,
    #[serde(default, rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<crate::models::Location>>,
}

impl Event {
    /// An event of a shipment
    pub fn new(event_code: String, event_time: String) -> Event {
        Event {
            event_code,
            event_time,
            location: None,
        }
    }
}


