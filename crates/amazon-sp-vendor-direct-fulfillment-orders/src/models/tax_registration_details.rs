/*
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TaxRegistrationDetails : Tax registration details of the entity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxRegistrationDetails {
    /// Tax registration type for the entity.
    #[serde(rename = "taxRegistrationType", skip_serializing_if = "Option::is_none")]
    pub tax_registration_type: Option<TaxRegistrationType>,
    /// Tax registration number for the party. For example, VAT ID.
    #[serde(rename = "taxRegistrationNumber")]
    pub tax_registration_number: String,
    #[serde(rename = "taxRegistrationAddress", skip_serializing_if = "Option::is_none")]
    pub tax_registration_address: Option<Box<crate::models::Address>>,
    /// Tax registration message that can be used for additional tax related details.
    #[serde(rename = "taxRegistrationMessages", skip_serializing_if = "Option::is_none")]
    pub tax_registration_messages: Option<String>,
}

impl TaxRegistrationDetails {
    /// Tax registration details of the entity.
    pub fn new(tax_registration_number: String) -> TaxRegistrationDetails {
        TaxRegistrationDetails {
            tax_registration_type: None,
            tax_registration_number,
            tax_registration_address: None,
            tax_registration_messages: None,
        }
    }
}

/// Tax registration type for the entity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxRegistrationType {
    #[serde(rename = "VAT")]
    VAT,
    #[serde(rename = "GST")]
    GST,
}

impl Default for TaxRegistrationType {
    fn default() -> TaxRegistrationType {
        Self::VAT
    }
}

