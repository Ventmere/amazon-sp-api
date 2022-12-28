/*
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Duration {
    /// Unit for duration.
    #[serde(rename = "durationUnit")]
    pub duration_unit: DurationUnit,
    /// Value for the duration in terms of the durationUnit.
    #[serde(rename = "durationValue")]
    pub duration_value: i32,
}

impl Duration {
    pub fn new(duration_unit: DurationUnit, duration_value: i32) -> Duration {
        Duration {
            duration_unit,
            duration_value,
        }
    }
}

/// Unit for duration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DurationUnit {
    #[serde(rename = "Days")]
    Days,
    #[serde(rename = "Months")]
    Months,
}

impl Default for DurationUnit {
    fn default() -> DurationUnit {
        Self::Days
    }
}
