/*
 * Selling Partner API for Messaging
 *
 * With the Messaging API you can build applications that send messages to buyers. You can get a list of message types that are available for an order that you specify, then call an operation that sends a message to the buyer for that order. The Messaging API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateWarrantyRequest : The request schema for the createWarranty operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateWarrantyRequest {
    /// Attachments to include in the message to the buyer. If any text is included in the attachment, the text must be written in the buyer's language of preference, which can be retrieved from the GetAttributes operation.
    #[serde(default, rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<crate::models::Attachment>>,
    /// The start date of the warranty coverage to include in the message to the buyer.
    #[serde(default, rename = "coverageStartDate", skip_serializing_if = "Option::is_none")]
    pub coverage_start_date: Option<String>,
    /// The end date of the warranty coverage to include in the message to the buyer.
    #[serde(default, rename = "coverageEndDate", skip_serializing_if = "Option::is_none")]
    pub coverage_end_date: Option<String>,
}

impl CreateWarrantyRequest {
    /// The request schema for the createWarranty operation.
    pub fn new() -> CreateWarrantyRequest {
        CreateWarrantyRequest {
            attachments: None,
            coverage_start_date: None,
            coverage_end_date: None,
        }
    }
}


