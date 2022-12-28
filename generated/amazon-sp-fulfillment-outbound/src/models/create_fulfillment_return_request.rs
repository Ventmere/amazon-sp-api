/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateFulfillmentReturnRequest : The createFulfillmentReturn operation creates a fulfillment return for items that were fulfilled using the createFulfillmentOrder operation. For calls to createFulfillmentReturn, you must include ReturnReasonCode values returned by a previous call to the listReturnReasonCodes operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFulfillmentReturnRequest {
    /// An array of items to be returned.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::CreateReturnItem>,
}

impl CreateFulfillmentReturnRequest {
    /// The createFulfillmentReturn operation creates a fulfillment return for items that were fulfilled using the createFulfillmentOrder operation. For calls to createFulfillmentReturn, you must include ReturnReasonCode values returned by a previous call to the listReturnReasonCodes operation.
    pub fn new(items: Vec<crate::models::CreateReturnItem>) -> CreateFulfillmentReturnRequest {
        CreateFulfillmentReturnRequest {
            items,
        }
    }
}

