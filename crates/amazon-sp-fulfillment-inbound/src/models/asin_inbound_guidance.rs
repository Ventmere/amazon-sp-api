/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AsinInboundGuidance : Reasons why a given ASIN is not recommended for shipment to Amazon's fulfillment network.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AsinInboundGuidance {
    /// The Amazon Standard Identification Number (ASIN) of the item.
    #[serde(rename = "ASIN")]
    pub ASIN: String,
    #[serde(rename = "InboundGuidance")]
    pub inbound_guidance: crate::models::InboundGuidance,
    /// A list of inbound guidance reason information.
    #[serde(rename = "GuidanceReasonList", skip_serializing_if = "Option::is_none")]
    pub guidance_reason_list: Option<Vec<crate::models::GuidanceReason>>,
}

impl AsinInboundGuidance {
    /// Reasons why a given ASIN is not recommended for shipment to Amazon's fulfillment network.
    pub fn new(ASIN: String, inbound_guidance: crate::models::InboundGuidance) -> AsinInboundGuidance {
        AsinInboundGuidance {
            ASIN,
            inbound_guidance,
            guidance_reason_list: None,
        }
    }
}


