/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetTransportDetailsResult {
    #[serde(default, rename = "TransportContent", skip_serializing_if = "Option::is_none")]
    pub transport_content: Option<Box<crate::models::TransportContent>>,
}

impl GetTransportDetailsResult {
    pub fn new() -> GetTransportDetailsResult {
        GetTransportDetailsResult {
            transport_content: None,
        }
    }
}


