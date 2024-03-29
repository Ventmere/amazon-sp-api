/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFulfillmentPreviewResult : A list of fulfillment order previews, including estimated shipping weights, estimated shipping fees, and estimated ship dates and arrival dates.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFulfillmentPreviewResult {
    /// An array of fulfillment preview information.
    #[serde(default, rename = "fulfillmentPreviews", skip_serializing_if = "Option::is_none")]
    pub fulfillment_previews: Option<Vec<crate::models::FulfillmentPreview>>,
}

impl GetFulfillmentPreviewResult {
    /// A list of fulfillment order previews, including estimated shipping weights, estimated shipping fees, and estimated ship dates and arrival dates.
    pub fn new() -> GetFulfillmentPreviewResult {
        GetFulfillmentPreviewResult {
            fulfillment_previews: None,
        }
    }
}


