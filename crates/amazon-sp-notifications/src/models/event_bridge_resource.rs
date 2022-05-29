/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EventBridgeResource : Represents an Amazon EventBridge destination.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EventBridgeResource {
    /// The name of the partner event source associated with the destination.
    #[serde(rename = "name")]
    pub name: String,
    /// The AWS region in which you receive the notifications. For AWS regions that are supported in Amazon EventBridge, see https://docs.aws.amazon.com/general/latest/gr/ev.html.
    #[serde(rename = "region")]
    pub region: String,
    /// The identifier for the AWS account that is responsible for charges related to receiving notifications.
    #[serde(rename = "accountId")]
    pub account_id: String,
}

impl EventBridgeResource {
    /// Represents an Amazon EventBridge destination.
    pub fn new(name: String, region: String, account_id: String) -> EventBridgeResource {
        EventBridgeResource {
            name,
            region,
            account_id,
        }
    }
}


