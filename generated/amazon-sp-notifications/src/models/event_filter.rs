/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EventFilter : A notificationType specific filter. This object contains all of the currently available filters and properties that you can use to define a notificationType specific filter.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EventFilter {
    #[serde(rename = "aggregationSettings", skip_serializing_if = "Option::is_none")]
    pub aggregation_settings: Option<Box<crate::models::AggregationSettings>>,
    /// A list of marketplace identifiers to subscribe to (e.g. ATVPDKIKX0DER). To receive notifications in every marketplace, do not provide this list.
    #[serde(rename = "marketplaceIds", skip_serializing_if = "Option::is_none")]
    pub marketplace_ids: Option<Vec<String>>,
    /// An eventFilterType value that is supported by the specific notificationType. This is used by the subscription service to determine the type of event filter. Refer to the section of the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide) that describes the specific notificationType to determine if an eventFilterType is supported.
    #[serde(rename = "eventFilterType")]
    pub event_filter_type: String,
}

impl EventFilter {
    /// A notificationType specific filter. This object contains all of the currently available filters and properties that you can use to define a notificationType specific filter.
    pub fn new(event_filter_type: String) -> EventFilter {
        EventFilter {
            aggregation_settings: None,
            marketplace_ids: None,
            event_filter_type,
        }
    }
}


