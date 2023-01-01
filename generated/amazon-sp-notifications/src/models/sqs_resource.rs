/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SqsResource : The information required to create an Amazon Simple Queue Service (Amazon SQS) queue destination.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SqsResource {
    /// The Amazon Resource Name (ARN) associated with the SQS queue.
    #[serde(default, rename = "arn")]
    pub arn: String,
}

impl SqsResource {
    /// The information required to create an Amazon Simple Queue Service (Amazon SQS) queue destination.
    pub fn new(arn: String) -> SqsResource {
        SqsResource {
            arn,
        }
    }
}


