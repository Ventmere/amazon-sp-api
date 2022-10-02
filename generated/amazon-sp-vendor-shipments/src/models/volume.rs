/*
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Volume : The volume of the container.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Volume {
    /// The unit of measurement.
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: UnitOfMeasure,
    /// A decimal number with no loss of precision. Useful when precision loss is unacceptable, as with currencies. Follows RFC7159 for number representation. <br>**Pattern** : `^-?(0|([1-9]\\d*))(\\.\\d+)?([eE][+-]?\\d+)?$`.
    #[serde(rename = "value")]
    pub value: String,
}

impl Volume {
    /// The volume of the container.
    pub fn new(unit_of_measure: UnitOfMeasure, value: String) -> Volume {
        Volume {
            unit_of_measure,
            value,
        }
    }
}

/// The unit of measurement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnitOfMeasure {
    #[serde(rename = "CuFt")]
    CuFt,
    #[serde(rename = "CuIn")]
    CuIn,
    #[serde(rename = "CuM")]
    CuM,
    #[serde(rename = "CuY")]
    CuY,
}

impl Default for UnitOfMeasure {
    fn default() -> UnitOfMeasure {
        Self::CuFt
    }
}

