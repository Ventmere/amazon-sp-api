/*
 * Selling Partner API for Messaging
 *
 * With the Messaging API you can build applications that send messages to buyers. You can get a list of message types that are available for an order that you specify, then call an operation that sends a message to the buyer for that order. The Messaging API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateWarrantyResponse : The response schema for the createWarranty operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateWarrantyResponse {
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl CreateWarrantyResponse {
    /// The response schema for the createWarranty operation.
    pub fn new() -> CreateWarrantyResponse {
        CreateWarrantyResponse {
            errors: None,
        }
    }
}


