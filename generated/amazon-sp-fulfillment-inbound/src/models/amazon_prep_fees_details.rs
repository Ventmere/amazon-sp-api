/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AmazonPrepFeesDetails : The fees for Amazon to prep goods for shipment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AmazonPrepFeesDetails {
    #[serde(default, rename = "PrepInstruction", skip_serializing_if = "Option::is_none")]
    pub prep_instruction: Option<crate::models::PrepInstruction>,
    #[serde(default, rename = "FeePerUnit", skip_serializing_if = "Option::is_none")]
    pub fee_per_unit: Option<Box<crate::models::Amount>>,
}

impl AmazonPrepFeesDetails {
    /// The fees for Amazon to prep goods for shipment.
    pub fn new() -> AmazonPrepFeesDetails {
        AmazonPrepFeesDetails {
            prep_instruction: None,
            fee_per_unit: None,
        }
    }
}


