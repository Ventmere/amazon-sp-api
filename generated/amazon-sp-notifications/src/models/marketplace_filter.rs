/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MarketplaceFilter : Use this event filter to customize your subscription to send notifications for only the specified marketplaceId's.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MarketplaceFilter {
    /// A list of marketplace identifiers to subscribe to (e.g. ATVPDKIKX0DER). To receive notifications in every marketplace, do not provide this list.
    #[serde(default, rename = "marketplaceIds", skip_serializing_if = "Option::is_none")]
    pub marketplace_ids: Option<Vec<String>>,
}

impl MarketplaceFilter {
    /// Use this event filter to customize your subscription to send notifications for only the specified marketplaceId's.
    pub fn new() -> MarketplaceFilter {
        MarketplaceFilter {
            marketplace_ids: None,
        }
    }
}


