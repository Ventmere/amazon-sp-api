/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateScheduleRequest : Request schema for the `updateSchedule` operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateScheduleRequest {
    /// List of `AvailabilityRecord`s to represent the capacity of a resource over a time range.
    #[serde(rename = "schedules")]
    pub schedules: Vec<crate::models::AvailabilityRecord>,
}

impl UpdateScheduleRequest {
    /// Request schema for the `updateSchedule` operation.
    pub fn new(schedules: Vec<crate::models::AvailabilityRecord>) -> UpdateScheduleRequest {
        UpdateScheduleRequest {
            schedules,
        }
    }
}

