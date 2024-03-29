/*
 * Selling Partner API for Reports
 *
 * The Selling Partner API for Reports lets you retrieve and manage a variety of reports that can help selling partners manage their businesses.
 *
 * The version of the OpenAPI document: 2020-09-04
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetReportScheduleResponse : The response for the getReportSchedule operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetReportScheduleResponse {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::ReportSchedule>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl GetReportScheduleResponse {
    /// The response for the getReportSchedule operation.
    pub fn new() -> GetReportScheduleResponse {
        GetReportScheduleResponse {
            payload: None,
            errors: None,
        }
    }
}


