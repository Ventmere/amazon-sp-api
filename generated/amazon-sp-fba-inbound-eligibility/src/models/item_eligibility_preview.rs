/*
 * Selling Partner API for FBA Inbound Eligibilty
 *
 * With the FBA Inbound Eligibility API, you can build applications that let sellers get eligibility previews for items before shipping them to Amazon's fulfillment centers. With this API you can find out if an item is eligible for inbound shipment to Amazon's fulfillment centers in a specific marketplace. You can also find out if an item is eligible for using the manufacturer barcode for FBA inventory tracking. Sellers can use this information to inform their decisions about which items to ship Amazon's fulfillment centers.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemEligibilityPreview : The response object which contains the ASIN, marketplaceId if required, eligibility program, the eligibility status (boolean), and a list of ineligibility reason codes.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemEligibilityPreview {
    /// The ASIN for which eligibility was determined.
    #[serde(rename = "asin")]
    pub asin: String,
    /// The marketplace for which eligibility was determined.
    #[serde(rename = "marketplaceId", skip_serializing_if = "Option::is_none")]
    pub marketplace_id: Option<String>,
    /// The program for which eligibility was determined.
    #[serde(rename = "program")]
    pub program: Program,
    /// Indicates if the item is eligible for the program.
    #[serde(rename = "isEligibleForProgram")]
    pub is_eligible_for_program: bool,
    /// Potential Ineligibility Reason Codes.
    #[serde(rename = "ineligibilityReasonList", skip_serializing_if = "Option::is_none")]
    pub ineligibility_reason_list: Option<Vec<IneligibilityReasonList>>,
}

impl ItemEligibilityPreview {
    /// The response object which contains the ASIN, marketplaceId if required, eligibility program, the eligibility status (boolean), and a list of ineligibility reason codes.
    pub fn new(asin: String, program: Program, is_eligible_for_program: bool) -> ItemEligibilityPreview {
        ItemEligibilityPreview {
            asin,
            marketplace_id: None,
            program,
            is_eligible_for_program,
            ineligibility_reason_list: None,
        }
    }
}

/// The program for which eligibility was determined.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Program {
    #[serde(rename = "INBOUND")]
    INBOUND,
    #[serde(rename = "COMMINGLING")]
    COMMINGLING,
}

impl Default for Program {
    fn default() -> Program {
        Self::INBOUND
    }
}
/// Potential Ineligibility Reason Codes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IneligibilityReasonList {
    #[serde(rename = "FBA_INB_0004")]
    FBAINB0004,
    #[serde(rename = "FBA_INB_0006")]
    FBAINB0006,
    #[serde(rename = "FBA_INB_0007")]
    FBAINB0007,
    #[serde(rename = "FBA_INB_0008")]
    FBAINB0008,
    #[serde(rename = "FBA_INB_0009")]
    FBAINB0009,
    #[serde(rename = "FBA_INB_0010")]
    FBAINB0010,
    #[serde(rename = "FBA_INB_0011")]
    FBAINB0011,
    #[serde(rename = "FBA_INB_0012")]
    FBAINB0012,
    #[serde(rename = "FBA_INB_0013")]
    FBAINB0013,
    #[serde(rename = "FBA_INB_0014")]
    FBAINB0014,
    #[serde(rename = "FBA_INB_0015")]
    FBAINB0015,
    #[serde(rename = "FBA_INB_0016")]
    FBAINB0016,
    #[serde(rename = "FBA_INB_0017")]
    FBAINB0017,
    #[serde(rename = "FBA_INB_0018")]
    FBAINB0018,
    #[serde(rename = "FBA_INB_0019")]
    FBAINB0019,
    #[serde(rename = "FBA_INB_0034")]
    FBAINB0034,
    #[serde(rename = "FBA_INB_0035")]
    FBAINB0035,
    #[serde(rename = "FBA_INB_0036")]
    FBAINB0036,
    #[serde(rename = "FBA_INB_0037")]
    FBAINB0037,
    #[serde(rename = "FBA_INB_0038")]
    FBAINB0038,
    #[serde(rename = "FBA_INB_0050")]
    FBAINB0050,
    #[serde(rename = "FBA_INB_0051")]
    FBAINB0051,
    #[serde(rename = "FBA_INB_0053")]
    FBAINB0053,
    #[serde(rename = "FBA_INB_0055")]
    FBAINB0055,
    #[serde(rename = "FBA_INB_0056")]
    FBAINB0056,
    #[serde(rename = "FBA_INB_0059")]
    FBAINB0059,
    #[serde(rename = "FBA_INB_0065")]
    FBAINB0065,
    #[serde(rename = "FBA_INB_0066")]
    FBAINB0066,
    #[serde(rename = "FBA_INB_0067")]
    FBAINB0067,
    #[serde(rename = "FBA_INB_0068")]
    FBAINB0068,
    #[serde(rename = "FBA_INB_0095")]
    FBAINB0095,
    #[serde(rename = "FBA_INB_0097")]
    FBAINB0097,
    #[serde(rename = "FBA_INB_0098")]
    FBAINB0098,
    #[serde(rename = "FBA_INB_0099")]
    FBAINB0099,
    #[serde(rename = "FBA_INB_0100")]
    FBAINB0100,
    #[serde(rename = "FBA_INB_0103")]
    FBAINB0103,
    #[serde(rename = "FBA_INB_0104")]
    FBAINB0104,
    #[serde(rename = "FBA_INB_0197")]
    FBAINB0197,
    #[serde(rename = "UNKNOWN_INB_ERROR_CODE")]
    UNKNOWNINBERRORCODE,
}

impl Default for IneligibilityReasonList {
    fn default() -> IneligibilityReasonList {
        Self::FBAINB0004
    }
}
