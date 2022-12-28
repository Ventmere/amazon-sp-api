/*
 * Selling Partner API for Reports
 *
 * The Selling Partner API for Reports lets you retrieve and manage a variety of reports that can help selling partners manage their businesses.
 *
 * The version of the OpenAPI document: 2020-09-04
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReportSchedule : Detailed information about a report schedule.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReportSchedule {
    /// The identifier for the report schedule. This identifier is unique only in combination with a seller ID.
    #[serde(rename = "reportScheduleId")]
    pub report_schedule_id: String,
    /// The report type.
    #[serde(rename = "reportType")]
    pub report_type: String,
    /// A list of marketplace identifiers. The report document's contents will contain data for all of the specified marketplaces, unless the report type indicates otherwise.
    #[serde(rename = "marketplaceIds", skip_serializing_if = "Option::is_none")]
    pub marketplace_ids: Option<Vec<String>>,
    /// Additional information passed to reports. This varies by report type.
    #[serde(rename = "reportOptions", skip_serializing_if = "Option::is_none")]
    pub report_options: Option<::std::collections::HashMap<String, String>>,
    /// An ISO 8601 period value that indicates how often a report should be created.
    #[serde(rename = "period")]
    pub period: String,
    /// The date and time when the schedule will create its next report, in ISO 8601 date time format.
    #[serde(rename = "nextReportCreationTime", skip_serializing_if = "Option::is_none")]
    pub next_report_creation_time: Option<String>,
}

impl ReportSchedule {
    /// Detailed information about a report schedule.
    pub fn new(report_schedule_id: String, report_type: String, period: String) -> ReportSchedule {
        ReportSchedule {
            report_schedule_id,
            report_type,
            marketplace_ids: None,
            report_options: None,
            period,
            next_report_creation_time: None,
        }
    }
}

