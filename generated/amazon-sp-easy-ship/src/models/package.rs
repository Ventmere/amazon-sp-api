/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// Package : A package. This object contains all the details of the scheduled Easy Ship package including the package identifier, physical attributes such as dimensions and weight, selected time slot to handover the package to carrier, status of the package, and tracking/invoice details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "scheduledPackageId")]
    pub scheduled_package_id: Box<crate::models::ScheduledPackageId>,
    #[serde(rename = "packageDimensions")]
    pub package_dimensions: Box<crate::models::Dimensions>,
    #[serde(rename = "packageWeight")]
    pub package_weight: Box<crate::models::Weight>,
    /// A list of items contained in the package.
    #[serde(rename = "packageItems", skip_serializing_if = "Option::is_none")]
    pub package_items: Option<Vec<crate::models::Item>>,
    #[serde(rename = "packageTimeSlot")]
    pub package_time_slot: Box<crate::models::TimeSlot>,
    /// Optional seller-created identifier that is printed on the shipping label to help the seller identify the package.
    #[serde(rename = "packageIdentifier", skip_serializing_if = "Option::is_none")]
    pub package_identifier: Option<String>,
    #[serde(rename = "invoice", skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Box<crate::models::InvoiceData>>,
    #[serde(rename = "packageStatus", skip_serializing_if = "Option::is_none")]
    pub package_status: Option<crate::models::PackageStatus>,
    #[serde(rename = "trackingDetails", skip_serializing_if = "Option::is_none")]
    pub tracking_details: Option<Box<crate::models::TrackingDetails>>,
}

impl Package {
    /// A package. This object contains all the details of the scheduled Easy Ship package including the package identifier, physical attributes such as dimensions and weight, selected time slot to handover the package to carrier, status of the package, and tracking/invoice details.
    pub fn new(scheduled_package_id: crate::models::ScheduledPackageId, package_dimensions: crate::models::Dimensions, package_weight: crate::models::Weight, package_time_slot: crate::models::TimeSlot) -> Package {
        Package {
            scheduled_package_id: Box::new(scheduled_package_id),
            package_dimensions: Box::new(package_dimensions),
            package_weight: Box::new(package_weight),
            package_items: None,
            package_time_slot: Box::new(package_time_slot),
            package_identifier: None,
            invoice: None,
            package_status: None,
            tracking_details: None,
        }
    }
}


