/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Dimensions : The dimension values and unit of measurement.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dimensions {
    #[serde(default, rename = "Length")]
    pub length: f64,
    #[serde(default, rename = "Width")]
    pub width: f64,
    #[serde(default, rename = "Height")]
    pub height: f64,
    #[serde(default, rename = "Unit")]
    pub unit: crate::models::UnitOfMeasurement,
}

impl Dimensions {
    /// The dimension values and unit of measurement.
    pub fn new(length: f64, width: f64, height: f64, unit: crate::models::UnitOfMeasurement) -> Dimensions {
        Dimensions {
            length,
            width,
            height,
            unit,
        }
    }
}


