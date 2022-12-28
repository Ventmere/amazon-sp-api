/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// ListHandoverSlotsRequest : The request schema for the `listHandoverSlots` operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListHandoverSlotsRequest {
    /// A string of up to 255 characters.
    #[serde(rename = "marketplaceId")]
    pub marketplace_id: String,
    /// An Amazon-defined order identifier. Identifies the order that the seller wants to deliver using Amazon Easy Ship.
    #[serde(rename = "amazonOrderId")]
    pub amazon_order_id: String,
    #[serde(rename = "packageDimensions")]
    pub package_dimensions: Box<crate::models::Dimensions>,
    #[serde(rename = "packageWeight")]
    pub package_weight: Box<crate::models::Weight>,
}

impl ListHandoverSlotsRequest {
    /// The request schema for the `listHandoverSlots` operation.
    pub fn new(marketplace_id: String, amazon_order_id: String, package_dimensions: crate::models::Dimensions, package_weight: crate::models::Weight) -> ListHandoverSlotsRequest {
        ListHandoverSlotsRequest {
            marketplace_id,
            amazon_order_id,
            package_dimensions: Box::new(package_dimensions),
            package_weight: Box::new(package_weight),
        }
    }
}

