/*
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartyIdentification {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<crate::models::Address>>,
    /// Assigned identification for the party.
    #[serde(rename = "partyId")]
    pub party_id: String,
    /// Tax registration details of the entity.
    #[serde(rename = "taxRegistrationDetails", skip_serializing_if = "Option::is_none")]
    pub tax_registration_details: Option<Vec<crate::models::TaxRegistrationDetails>>,
}

impl PartyIdentification {
    pub fn new(party_id: String) -> PartyIdentification {
        PartyIdentification {
            address: None,
            party_id,
            tax_registration_details: None,
        }
    }
}


