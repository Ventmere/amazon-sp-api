/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// Dimensions : The dimensions of the scheduled package.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dimensions {
    /// The numerical value of the specified dimension.
    #[serde(default, rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<f32>,
    /// The numerical value of the specified dimension.
    #[serde(default, rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
    /// The numerical value of the specified dimension.
    #[serde(default, rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    #[serde(default, rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<crate::models::UnitOfLength>,
    /// A string of up to 255 characters.
    #[serde(default, rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl Dimensions {
    /// The dimensions of the scheduled package.
    pub fn new() -> Dimensions {
        Dimensions {
            length: None,
            width: None,
            height: None,
            unit: None,
            identifier: None,
        }
    }
}


