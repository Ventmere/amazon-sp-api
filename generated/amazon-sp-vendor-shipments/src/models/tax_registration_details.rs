/*
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TaxRegistrationDetails : Tax registration details of the entity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxRegistrationDetails {
    /// Tax registration type for the entity.
    #[serde(default, rename = "taxRegistrationType")]
    pub tax_registration_type: TaxRegistrationType,
    /// Tax registration number for the entity. For example, VAT ID.
    #[serde(default, rename = "taxRegistrationNumber")]
    pub tax_registration_number: String,
}

impl TaxRegistrationDetails {
    /// Tax registration details of the entity.
    pub fn new(tax_registration_type: TaxRegistrationType, tax_registration_number: String) -> TaxRegistrationDetails {
        TaxRegistrationDetails {
            tax_registration_type,
            tax_registration_number,
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

