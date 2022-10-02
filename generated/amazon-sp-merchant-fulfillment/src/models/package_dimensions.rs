/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PackageDimensions : The dimensions of a package contained in a shipment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PackageDimensions {
    #[serde(rename = "Length", skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<crate::models::UnitOfLength>,
    #[serde(rename = "PredefinedPackageDimensions", skip_serializing_if = "Option::is_none")]
    pub predefined_package_dimensions: Option<crate::models::PredefinedPackageDimensions>,
}

impl PackageDimensions {
    /// The dimensions of a package contained in a shipment.
    pub fn new() -> PackageDimensions {
        PackageDimensions {
            length: None,
            width: None,
            height: None,
            unit: None,
            predefined_package_dimensions: None,
        }
    }
}

