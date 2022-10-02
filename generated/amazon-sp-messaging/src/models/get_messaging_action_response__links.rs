/*
 * Selling Partner API for Messaging
 *
 * With the Messaging API you can build applications that send messages to buyers. You can get a list of message types that are available for an order that you specify, then call an operation that sends a message to the buyer for that order. The Messaging API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMessagingActionResponseLinks {
    #[serde(rename = "self")]
    pub _self: Box<crate::models::LinkObject>,
    #[serde(rename = "schema")]
    pub schema: Box<crate::models::LinkObject>,
}

impl GetMessagingActionResponseLinks {
    pub fn new(_self: crate::models::LinkObject, schema: crate::models::LinkObject) -> GetMessagingActionResponseLinks {
        GetMessagingActionResponseLinks {
            _self: Box::new(_self),
            schema: Box::new(schema),
        }
    }
}


