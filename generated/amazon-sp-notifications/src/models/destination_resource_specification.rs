/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DestinationResourceSpecification : The information required to create a destination resource. Applications should use one resource type (sqs or eventBridge) per destination.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinationResourceSpecification {
    #[serde(default, rename = "sqs", skip_serializing_if = "Option::is_none")]
    pub sqs: Option<Box<crate::models::SqsResource>>,
    #[serde(default, rename = "eventBridge", skip_serializing_if = "Option::is_none")]
    pub event_bridge: Option<Box<crate::models::EventBridgeResourceSpecification>>,
}

impl DestinationResourceSpecification {
    /// The information required to create a destination resource. Applications should use one resource type (sqs or eventBridge) per destination.
    pub fn new() -> DestinationResourceSpecification {
        DestinationResourceSpecification {
            sqs: None,
            event_bridge: None,
        }
    }
}


