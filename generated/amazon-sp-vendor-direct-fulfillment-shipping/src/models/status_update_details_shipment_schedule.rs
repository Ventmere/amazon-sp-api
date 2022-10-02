/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StatusUpdateDetailsShipmentSchedule {
    /// Date on which the shipment is expected to reach the customer delivery location. This field is expected to be in ISO-8601 date/time format, with UTC time zone or UTC offset. For example, 2020-07-16T23:00:00Z or 2020-07-16T23:00:00+01:00.
    #[serde(rename = "estimatedDeliveryDateTime", skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_date_time: Option<String>,
    /// This field indicates the date and time at the start of the appointment window scheduled to deliver the shipment. This field is expected to be in ISO-8601 date/time format, with UTC time zone or UTC offset. For example, 2020-07-16T23:00:00Z or 2020-07-16T23:00:00+01:00.
    #[serde(rename = "apptWindowStartDateTime", skip_serializing_if = "Option::is_none")]
    pub appt_window_start_date_time: Option<String>,
    /// This field indicates the date and time at the end of the appointment window scheduled to deliver the shipment. This field is expected to be in ISO-8601 date/time format, with UTC time zone or UTC offset. For example, 2020-07-16T23:00:00Z or 2020-07-16T23:00:00+01:00.
    #[serde(rename = "apptWindowEndDateTime", skip_serializing_if = "Option::is_none")]
    pub appt_window_end_date_time: Option<String>,
}

impl StatusUpdateDetailsShipmentSchedule {
    pub fn new() -> StatusUpdateDetailsShipmentSchedule {
        StatusUpdateDetailsShipmentSchedule {
            estimated_delivery_date_time: None,
            appt_window_start_date_time: None,
            appt_window_end_date_time: None,
        }
    }
}


