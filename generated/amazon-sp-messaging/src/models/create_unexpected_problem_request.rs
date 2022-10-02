/*
 * Selling Partner API for Messaging
 *
 * With the Messaging API you can build applications that send messages to buyers. You can get a list of message types that are available for an order that you specify, then call an operation that sends a message to the buyer for that order. The Messaging API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateUnexpectedProblemRequest : The request schema for the createUnexpectedProblem operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateUnexpectedProblemRequest {
    /// The text to be sent to the buyer. Only links related to unexpected problem calls are allowed. Do not include HTML or email addresses. The text must be written in the buyer's language of preference, which can be retrieved from the GetAttributes operation.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl CreateUnexpectedProblemRequest {
    /// The request schema for the createUnexpectedProblem operation.
    pub fn new() -> CreateUnexpectedProblemRequest {
        CreateUnexpectedProblemRequest {
            text: None,
        }
    }
}


