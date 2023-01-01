/*
 * Selling Partner API for FBA Inventory
 *
 * The Selling Partner API for FBA Inventory lets you programmatically retrieve information about inventory in Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResearchingQuantity : The number of misplaced or warehouse damaged units that are actively being confirmed at our fulfillment centers.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResearchingQuantity {
    /// The total number of units currently being researched in Amazon's fulfillment network.
    #[serde(default, rename = "totalResearchingQuantity", skip_serializing_if = "Option::is_none")]
    pub total_researching_quantity: Option<i32>,
    /// A list of quantity details for items currently being researched.
    #[serde(default, rename = "researchingQuantityBreakdown", skip_serializing_if = "Option::is_none")]
    pub researching_quantity_breakdown: Option<Vec<crate::models::ResearchingQuantityEntry>>,
}

impl ResearchingQuantity {
    /// The number of misplaced or warehouse damaged units that are actively being confirmed at our fulfillment centers.
    pub fn new() -> ResearchingQuantity {
        ResearchingQuantity {
            total_researching_quantity: None,
            researching_quantity_breakdown: None,
        }
    }
}


