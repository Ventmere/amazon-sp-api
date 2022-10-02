/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InboundGuidance : Specific inbound guidance for an item.

/// Specific inbound guidance for an item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InboundGuidance {
    #[serde(rename = "InboundNotRecommended")]
    InboundNotRecommended,
    #[serde(rename = "InboundOK")]
    InboundOK,

}

impl ToString for InboundGuidance {
    fn to_string(&self) -> String {
        match self {
            Self::InboundNotRecommended => String::from("InboundNotRecommended"),
            Self::InboundOK => String::from("InboundOK"),
        }
    }
}

impl Default for InboundGuidance {
    fn default() -> InboundGuidance {
        Self::InboundNotRecommended
    }
}



