/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// Code : An error code that identifies the type of error that occurred. The error codes listed below are specific to the Easy Ship section.

/// An error code that identifies the type of error that occurred. The error codes listed below are specific to the Easy Ship section.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "InvalidInput")]
    InvalidInput,
    #[serde(rename = "InvalidTimeSlotId")]
    InvalidTimeSlotId,
    #[serde(rename = "ScheduledPackageAlreadyExists")]
    ScheduledPackageAlreadyExists,
    #[serde(rename = "ScheduleWindowExpired")]
    ScheduleWindowExpired,
    #[serde(rename = "RetryableAfterGettingNewSlots")]
    RetryableAfterGettingNewSlots,
    #[serde(rename = "TimeSlotNotAvailable")]
    TimeSlotNotAvailable,
    #[serde(rename = "ResourceNotFound")]
    ResourceNotFound,
    #[serde(rename = "InvalidOrderState")]
    InvalidOrderState,
    #[serde(rename = "RegionNotSupported")]
    RegionNotSupported,
    #[serde(rename = "OrderNotEligibleForRescheduling")]
    OrderNotEligibleForRescheduling,
    #[serde(rename = "InternalServerError")]
    InternalServerError,

}

impl ToString for Code {
    fn to_string(&self) -> String {
        match self {
            Self::InvalidInput => String::from("InvalidInput"),
            Self::InvalidTimeSlotId => String::from("InvalidTimeSlotId"),
            Self::ScheduledPackageAlreadyExists => String::from("ScheduledPackageAlreadyExists"),
            Self::ScheduleWindowExpired => String::from("ScheduleWindowExpired"),
            Self::RetryableAfterGettingNewSlots => String::from("RetryableAfterGettingNewSlots"),
            Self::TimeSlotNotAvailable => String::from("TimeSlotNotAvailable"),
            Self::ResourceNotFound => String::from("ResourceNotFound"),
            Self::InvalidOrderState => String::from("InvalidOrderState"),
            Self::RegionNotSupported => String::from("RegionNotSupported"),
            Self::OrderNotEligibleForRescheduling => String::from("OrderNotEligibleForRescheduling"),
            Self::InternalServerError => String::from("InternalServerError"),
        }
    }
}

impl Default for Code {
    fn default() -> Code {
        Self::InvalidInput
    }
}



