/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IntegerWithUnits : A whole number dimension and its unit of measurement. For example, this can represent 100 pixels.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IntegerWithUnits {
    /// The dimension value.
    #[serde(default, rename = "value")]
    pub value: i32,
    /// The unit of measurement.
    #[serde(default, rename = "units")]
    pub units: String,
}

impl IntegerWithUnits {
    /// A whole number dimension and its unit of measurement. For example, this can represent 100 pixels.
    pub fn new(value: i32, units: String) -> IntegerWithUnits {
        IntegerWithUnits {
            value,
            units,
        }
    }
}


