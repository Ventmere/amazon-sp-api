/*
 * Selling Partner API for FBA Inventory
 *
 * The Selling Partner API for FBA Inventory lets you programmatically retrieve information about inventory in Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResearchingQuantityEntry : The misplaced or warehouse damaged inventory that is actively being confirmed at our fulfillment centers.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResearchingQuantityEntry {
    /// The duration of the research.
    #[serde(rename = "name")]
    pub name: Name,
    /// The number of units.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl ResearchingQuantityEntry {
    /// The misplaced or warehouse damaged inventory that is actively being confirmed at our fulfillment centers.
    pub fn new(name: Name, quantity: i32) -> ResearchingQuantityEntry {
        ResearchingQuantityEntry {
            name,
            quantity,
        }
    }
}

/// The duration of the research.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "researchingQuantityInShortTerm")]
    ResearchingQuantityInShortTerm,
    #[serde(rename = "researchingQuantityInMidTerm")]
    ResearchingQuantityInMidTerm,
    #[serde(rename = "researchingQuantityInLongTerm")]
    ResearchingQuantityInLongTerm,
}

impl Default for Name {
    fn default() -> Name {
        Self::ResearchingQuantityInShortTerm
    }
}
