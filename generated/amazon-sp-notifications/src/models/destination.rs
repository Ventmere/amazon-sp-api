/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Destination : Represents a destination created when you call the createDestination operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Destination {
    /// The developer-defined name for this destination.
    #[serde(default, rename = "name")]
    pub name: String,
    /// The destination identifier generated when you created the destination.
    #[serde(default, rename = "destinationId")]
    pub destination_id: String,
    #[serde(default, rename = "resource")]
    pub resource: Box<crate::models::DestinationResource>,
}

impl Destination {
    /// Represents a destination created when you call the createDestination operation.
    pub fn new(name: String, destination_id: String, resource: crate::models::DestinationResource) -> Destination {
        Destination {
            name,
            destination_id,
            resource: Box::new(resource),
        }
    }
}


