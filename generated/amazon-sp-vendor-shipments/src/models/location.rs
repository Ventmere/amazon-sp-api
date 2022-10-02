/*
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Location : Location identifier.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Location {
    /// Type of location identification.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Location code.
    #[serde(rename = "locationCode", skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,
    /// The two digit country code. In ISO 3166-1 alpha-2 format.
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

impl Location {
    /// Location identifier.
    pub fn new() -> Location {
        Location {
            _type: None,
            location_code: None,
            country_code: None,
        }
    }
}


