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
pub struct CreateReportSpecification {
    /// Additional information passed to reports. This varies by report type.
    #[serde(default, rename = "reportOptions", skip_serializing_if = "Option::is_none")]
    pub report_options: Option<::std::collections::HashMap<String, String>>,
    /// The report type.
    #[serde(default, rename = "reportType")]
    pub report_type: String,
    /// The start of a date and time range, in ISO 8601 date time format, used for selecting the data to report. The default is now. The value must be prior to or equal to the current date and time. Not all report types make use of this.
    #[serde(default, rename = "dataStartTime", skip_serializing_if = "Option::is_none")]
    pub data_start_time: Option<String>,
    /// The end of a date and time range, in ISO 8601 date time format, used for selecting the data to report. The default is now. The value must be prior to or equal to the current date and time. Not all report types make use of this.
    #[serde(default, rename = "dataEndTime", skip_serializing_if = "Option::is_none")]
    pub data_end_time: Option<String>,
    /// A list of marketplace identifiers. The report document's contents will contain data for all of the specified marketplaces, unless the report type indicates otherwise.
    #[serde(default, rename = "marketplaceIds")]
    pub marketplace_ids: Vec<String>,
}

impl CreateReportSpecification {
    pub fn new(report_type: String, marketplace_ids: Vec<String>) -> CreateReportSpecification {
        CreateReportSpecification {
            report_options: None,
            report_type,
            data_start_time: None,
            data_end_time: None,
            marketplace_ids,
        }
    }
}


