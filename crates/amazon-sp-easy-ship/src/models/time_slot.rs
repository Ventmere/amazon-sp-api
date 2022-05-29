/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easy-ship-api-v2022-03-23-use-case-guide) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// TimeSlot : A time window to hand over an Easy Ship package to Amazon Logistics.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TimeSlot {
    /// A string of up to 255 characters.
    #[serde(rename = "slotId")]
    pub slot_id: String,
    /// A datetime value in ISO 8601 format.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// A datetime value in ISO 8601 format.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "handoverMethod", skip_serializing_if = "Option::is_none")]
    pub handover_method: Option<crate::models::HandoverMethod>,
}

impl TimeSlot {
    /// A time window to hand over an Easy Ship package to Amazon Logistics.
    pub fn new(slot_id: String) -> TimeSlot {
        TimeSlot {
            slot_id,
            start_time: None,
            end_time: None,
            handover_method: None,
        }
    }
}


