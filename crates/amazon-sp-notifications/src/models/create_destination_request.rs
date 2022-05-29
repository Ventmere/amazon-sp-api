/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateDestinationRequest : The request schema for the createDestination operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateDestinationRequest {
    #[serde(rename = "resourceSpecification")]
    pub resource_specification: Box<crate::models::DestinationResourceSpecification>,
    /// A developer-defined name to help identify this destination.
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateDestinationRequest {
    /// The request schema for the createDestination operation.
    pub fn new(resource_specification: crate::models::DestinationResourceSpecification, name: String) -> CreateDestinationRequest {
        CreateDestinationRequest {
            resource_specification: Box::new(resource_specification),
            name,
        }
    }
}


