/*
 * Selling Partner API for Retail Procurement Payments
 *
 * The Selling Partner API for Retail Procurement Payments provides programmatic access to vendors payments data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AdditionalDetails : Additional information provided by the selling party for tax-related or any other purpose.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AdditionalDetails {
    /// The type of the additional information provided by the selling party.
    #[serde(default, rename = "type")]
    pub _type: Type,
    /// The detail of the additional information provided by the selling party.
    #[serde(default, rename = "detail")]
    pub detail: String,
    /// The language code of the additional information detail.
    #[serde(default, rename = "languageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl AdditionalDetails {
    /// Additional information provided by the selling party for tax-related or any other purpose.
    pub fn new(_type: Type, detail: String) -> AdditionalDetails {
        AdditionalDetails {
            _type,
            detail,
            language_code: None,
        }
    }
}

/// The type of the additional information provided by the selling party.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SUR")]
    SUR,
    #[serde(rename = "OCR")]
    OCR,
    #[serde(rename = "CartonCount")]
    CartonCount,
}

impl Default for Type {
    fn default() -> Type {
        Self::SUR
    }
}

