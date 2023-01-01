/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// Packages : A list of packages.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Packages {
    #[serde(default, rename = "packages")]
    pub packages: Vec<crate::models::Package>,
}

impl Packages {
    /// A list of packages.
    pub fn new(packages: Vec<crate::models::Package>) -> Packages {
        Packages {
            packages,
        }
    }
}


