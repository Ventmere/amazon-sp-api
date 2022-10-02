/*
 * Selling Partner API for Messaging
 *
 * With the Messaging API you can build applications that send messages to buyers. You can get a list of message types that are available for an order that you specify, then call an operation that sends a message to the buyer for that order. The Messaging API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvoiceRequest : The request schema for the sendInvoice operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceRequest {
    /// Attachments to include in the message to the buyer.
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<crate::models::Attachment>>,
}

impl InvoiceRequest {
    /// The request schema for the sendInvoice operation.
    pub fn new() -> InvoiceRequest {
        InvoiceRequest {
            attachments: None,
        }
    }
}


