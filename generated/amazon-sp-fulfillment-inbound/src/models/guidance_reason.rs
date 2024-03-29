/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GuidanceReason : A reason for the current inbound guidance for an item.

/// A reason for the current inbound guidance for an item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GuidanceReason {
    #[serde(rename = "SlowMovingASIN")]
    SlowMovingASIN,
    #[serde(rename = "NoApplicableGuidance")]
    NoApplicableGuidance,

}

impl ToString for GuidanceReason {
    fn to_string(&self) -> String {
        match self {
            Self::SlowMovingASIN => String::from("SlowMovingASIN"),
            Self::NoApplicableGuidance => String::from("NoApplicableGuidance"),
        }
    }
}

impl Default for GuidanceReason {
    fn default() -> GuidanceReason {
        Self::SlowMovingASIN
    }
}




