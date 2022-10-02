/*
 * Selling Partner API for Direct Fulfillment Payments
 *
 * The Selling Partner API for Direct Fulfillment Payments provides programmatic access to a direct fulfillment vendor's invoice data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TaxRegistrationDetail : Tax registration details of the entity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxRegistrationDetail {
    /// Tax registration type for the entity.
    #[serde(rename = "taxRegistrationType", skip_serializing_if = "Option::is_none")]
    pub tax_registration_type: Option<TaxRegistrationType>,
    /// Tax registration number for the party. For example, VAT ID.
    #[serde(rename = "taxRegistrationNumber")]
    pub tax_registration_number: String,
    #[serde(rename = "taxRegistrationAddress", skip_serializing_if = "Option::is_none")]
    pub tax_registration_address: Option<Box<crate::models::Address>>,
    /// Tax registration message that can be used for additional tax related details.
    #[serde(rename = "taxRegistrationMessage", skip_serializing_if = "Option::is_none")]
    pub tax_registration_message: Option<String>,
}

impl TaxRegistrationDetail {
    /// Tax registration details of the entity.
    pub fn new(tax_registration_number: String) -> TaxRegistrationDetail {
        TaxRegistrationDetail {
            tax_registration_type: None,
            tax_registration_number,
            tax_registration_address: None,
            tax_registration_message: None,
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

