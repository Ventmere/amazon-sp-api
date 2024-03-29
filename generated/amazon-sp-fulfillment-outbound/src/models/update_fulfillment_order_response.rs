/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateFulfillmentOrderResponse : The response schema for the updateFulfillmentOrder operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateFulfillmentOrderResponse {
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl UpdateFulfillmentOrderResponse {
    /// The response schema for the updateFulfillmentOrder operation.
    pub fn new() -> UpdateFulfillmentOrderResponse {
        UpdateFulfillmentOrderResponse {
            errors: None,
        }
    }
}


