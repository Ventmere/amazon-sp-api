/*
 * Selling Partner API for Messaging
 *
 * With the Messaging API you can build applications that send messages to buyers. You can get a list of message types that are available for an order that you specify, then call an operation that sends a message to the buyer for that order. The Messaging API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetMessagingActionResponse : Describes a messaging action that can be taken for an order. Provides a JSON Hypertext Application Language (HAL) link to the JSON schema document that describes the expected input.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMessagingActionResponse {
    #[serde(default, rename = "_links", skip_serializing_if = "Option::is_none")]
    pub _links: Option<Box<crate::models::GetMessagingActionResponseLinks>>,
    #[serde(default, rename = "_embedded", skip_serializing_if = "Option::is_none")]
    pub _embedded: Option<Box<crate::models::GetMessagingActionResponseEmbedded>>,
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::MessagingAction>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetMessagingActionResponse {
    /// Describes a messaging action that can be taken for an order. Provides a JSON Hypertext Application Language (HAL) link to the JSON schema document that describes the expected input.
    pub fn new() -> GetMessagingActionResponse {
        GetMessagingActionResponse {
            _links: None,
            _embedded: None,
            payload: None,
            errors: None,
        }
    }
}


