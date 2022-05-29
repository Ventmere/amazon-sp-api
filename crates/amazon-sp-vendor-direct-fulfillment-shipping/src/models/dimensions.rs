/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Dimensions : Physical dimensional measurements of a container.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dimensions {
    /// A decimal number with no loss of precision. Useful when precision loss is unacceptable, as with currencies. Follows RFC7159 for number representation.  <br>**Pattern** : `^-?(0|([1-9]\\\\d*))(\\\\.\\\\d+)?([eE][+-]?\\\\d+)?$`.
    #[serde(rename = "length")]
    pub length: String,
    /// A decimal number with no loss of precision. Useful when precision loss is unacceptable, as with currencies. Follows RFC7159 for number representation.  <br>**Pattern** : `^-?(0|([1-9]\\\\d*))(\\\\.\\\\d+)?([eE][+-]?\\\\d+)?$`.
    #[serde(rename = "width")]
    pub width: String,
    /// A decimal number with no loss of precision. Useful when precision loss is unacceptable, as with currencies. Follows RFC7159 for number representation.  <br>**Pattern** : `^-?(0|([1-9]\\\\d*))(\\\\.\\\\d+)?([eE][+-]?\\\\d+)?$`.
    #[serde(rename = "height")]
    pub height: String,
    /// The unit of measure for dimensions.
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: UnitOfMeasure,
}

impl Dimensions {
    /// Physical dimensional measurements of a container.
    pub fn new(length: String, width: String, height: String, unit_of_measure: UnitOfMeasure) -> Dimensions {
        Dimensions {
            length,
            width,
            height,
            unit_of_measure,
        }
    }
}

/// The unit of measure for dimensions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnitOfMeasure {
    #[serde(rename = "IN")]
    _IN,
    #[serde(rename = "CM")]
    CM,
}

impl Default for UnitOfMeasure {
    fn default() -> UnitOfMeasure {
        Self::_IN
    }
}

