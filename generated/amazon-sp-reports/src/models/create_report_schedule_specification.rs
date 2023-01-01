/*
 * Selling Partner API for Reports
 *
 * The Selling Partner API for Reports lets you retrieve and manage a variety of reports that can help selling partners manage their businesses.
 *
 * The version of the OpenAPI document: 2020-09-04
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateReportScheduleSpecification {
    /// The report type.
    #[serde(default, rename = "reportType")]
    pub report_type: String,
    /// A list of marketplace identifiers for the report schedule.
    #[serde(default, rename = "marketplaceIds")]
    pub marketplace_ids: Vec<String>,
    /// Additional information passed to reports. This varies by report type.
    #[serde(default, rename = "reportOptions", skip_serializing_if = "Option::is_none")]
    pub report_options: Option<::std::collections::HashMap<String, String>>,
    /// One of a set of predefined ISO 8601 periods that specifies how often a report should be created.
    #[serde(default, rename = "period")]
    pub period: Period,
    /// The date and time when the schedule will create its next report, in ISO 8601 date time format.
    #[serde(default, rename = "nextReportCreationTime", skip_serializing_if = "Option::is_none")]
    pub next_report_creation_time: Option<String>,
}

impl CreateReportScheduleSpecification {
    pub fn new(report_type: String, marketplace_ids: Vec<String>, period: Period) -> CreateReportScheduleSpecification {
        CreateReportScheduleSpecification {
            report_type,
            marketplace_ids,
            report_options: None,
            period,
            next_report_creation_time: None,
        }
    }
}

/// One of a set of predefined ISO 8601 periods that specifies how often a report should be created.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Period {
    #[serde(rename = "PT5M")]
    PT5M,
    #[serde(rename = "PT15M")]
    PT15M,
    #[serde(rename = "PT30M")]
    PT30M,
    #[serde(rename = "PT1H")]
    PT1H,
    #[serde(rename = "PT2H")]
    PT2H,
    #[serde(rename = "PT4H")]
    PT4H,
    #[serde(rename = "PT8H")]
    PT8H,
    #[serde(rename = "PT12H")]
    PT12H,
    #[serde(rename = "P1D")]
    P1D,
    #[serde(rename = "P2D")]
    P2D,
    #[serde(rename = "P3D")]
    P3D,
    #[serde(rename = "PT84H")]
    PT84H,
    #[serde(rename = "P7D")]
    P7D,
    #[serde(rename = "P14D")]
    P14D,
    #[serde(rename = "P15D")]
    P15D,
    #[serde(rename = "P18D")]
    P18D,
    #[serde(rename = "P30D")]
    P30D,
    #[serde(rename = "P1M")]
    P1M,
}

impl Default for Period {
    fn default() -> Period {
        Self::PT5M
    }
}

