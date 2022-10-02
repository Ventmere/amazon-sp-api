/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeleteSubscriptionByIdResponse : The response schema for the deleteSubscriptionById operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteSubscriptionByIdResponse {
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl DeleteSubscriptionByIdResponse {
    /// The response schema for the deleteSubscriptionById operation.
    pub fn new() -> DeleteSubscriptionByIdResponse {
        DeleteSubscriptionByIdResponse {
            errors: None,
        }
    }
}

