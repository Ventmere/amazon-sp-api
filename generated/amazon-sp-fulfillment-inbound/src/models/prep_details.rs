/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PrepDetails : Preparation instructions and who is responsible for the preparation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PrepDetails {
    #[serde(default, rename = "PrepInstruction")]
    pub prep_instruction: crate::models::PrepInstruction,
    #[serde(default, rename = "PrepOwner")]
    pub prep_owner: crate::models::PrepOwner,
}

impl PrepDetails {
    /// Preparation instructions and who is responsible for the preparation.
    pub fn new(prep_instruction: crate::models::PrepInstruction, prep_owner: crate::models::PrepOwner) -> PrepDetails {
        PrepDetails {
            prep_instruction,
            prep_owner,
        }
    }
}


