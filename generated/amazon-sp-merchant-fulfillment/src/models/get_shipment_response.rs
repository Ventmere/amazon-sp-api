/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetShipmentResponse : Response schema.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetShipmentResponse {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::Shipment>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetShipmentResponse {
    /// Response schema.
    pub fn new() -> GetShipmentResponse {
        GetShipmentResponse {
            payload: None,
            errors: None,
        }
    }
}


