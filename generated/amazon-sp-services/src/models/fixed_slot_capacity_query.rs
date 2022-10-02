/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FixedSlotCapacityQuery : Request schema for the `getFixedSlotCapacity` operation. This schema is used to define the time range, capacity types and slot duration which are being queried.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FixedSlotCapacityQuery {
    /// An array of capacity types which are being requested. Default value is `[SCHEDULED_CAPACITY]`.
    #[serde(rename = "capacityTypes", skip_serializing_if = "Option::is_none")]
    pub capacity_types: Option<Vec<crate::models::CapacityType>>,
    /// Size in which slots are being requested. This value should be a multiple of 5 and fall in the range: 5 <= `slotDuration` <= 360.
    #[serde(rename = "slotDuration", skip_serializing_if = "Option::is_none")]
    pub slot_duration: Option<f32>,
    /// Start date time from which the capacity slots are being requested in ISO 8601 format.
    #[serde(rename = "startDateTime")]
    pub start_date_time: String,
    /// End date time up to which the capacity slots are being requested in ISO 8601 format.
    #[serde(rename = "endDateTime")]
    pub end_date_time: String,
}

impl FixedSlotCapacityQuery {
    /// Request schema for the `getFixedSlotCapacity` operation. This schema is used to define the time range, capacity types and slot duration which are being queried.
    pub fn new(start_date_time: String, end_date_time: String) -> FixedSlotCapacityQuery {
        FixedSlotCapacityQuery {
            capacity_types: None,
            slot_duration: None,
            start_date_time,
            end_date_time,
        }
    }
}

