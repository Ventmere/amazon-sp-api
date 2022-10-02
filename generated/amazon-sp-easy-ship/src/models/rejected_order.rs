/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// RejectedOrder : A order which we couldn't schedule on your behalf. It contains its id, and information on the error.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RejectedOrder {
    /// An Amazon-defined order identifier. Identifies the order that the seller wants to deliver using Amazon Easy Ship.
    #[serde(rename = "amazonOrderId")]
    pub amazon_order_id: String,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::Error>>,
}

impl RejectedOrder {
    /// A order which we couldn't schedule on your behalf. It contains its id, and information on the error.
    pub fn new(amazon_order_id: String) -> RejectedOrder {
        RejectedOrder {
            amazon_order_id,
            error: None,
        }
    }
}


