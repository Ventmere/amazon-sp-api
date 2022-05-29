/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AggregationSettings : A container that holds all of the necessary properties to configure the aggregation of notifications.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AggregationSettings {
    #[serde(rename = "aggregationTimePeriod")]
    pub aggregation_time_period: crate::models::AggregationTimePeriod,
}

impl AggregationSettings {
    /// A container that holds all of the necessary properties to configure the aggregation of notifications.
    pub fn new(aggregation_time_period: crate::models::AggregationTimePeriod) -> AggregationSettings {
        AggregationSettings {
            aggregation_time_period,
        }
    }
}


