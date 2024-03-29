/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AggregationFilter : Use this filter to select the aggregation time period at which to send notifications (e.g. limit to one notification every five minutes for high frequency notifications).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AggregationFilter {
    #[serde(default, rename = "aggregationSettings", skip_serializing_if = "Option::is_none")]
    pub aggregation_settings: Option<Box<crate::models::AggregationSettings>>,
}

impl AggregationFilter {
    /// Use this filter to select the aggregation time period at which to send notifications (e.g. limit to one notification every five minutes for high frequency notifications).
    pub fn new() -> AggregationFilter {
        AggregationFilter {
            aggregation_settings: None,
        }
    }
}


