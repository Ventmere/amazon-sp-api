/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ImagingServicesFeeEvent : A fee event related to Amazon Imaging services.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImagingServicesFeeEvent {
    /// The identifier for the imaging services request.
    #[serde(rename = "ImagingRequestBillingItemID", skip_serializing_if = "Option::is_none")]
    pub imaging_request_billing_item_id: Option<String>,
    /// The Amazon Standard Identification Number (ASIN) of the item for which the imaging service was requested.
    #[serde(rename = "ASIN", skip_serializing_if = "Option::is_none")]
    pub ASIN: Option<String>,
    #[serde(rename = "PostedDate", skip_serializing_if = "Option::is_none")]
    pub posted_date: Option<String>,
    /// A list of fee component information.
    #[serde(rename = "FeeList", skip_serializing_if = "Option::is_none")]
    pub fee_list: Option<Vec<crate::models::FeeComponent>>,
}

impl ImagingServicesFeeEvent {
    /// A fee event related to Amazon Imaging services.
    pub fn new() -> ImagingServicesFeeEvent {
        ImagingServicesFeeEvent {
            imaging_request_billing_item_id: None,
            ASIN: None,
            posted_date: None,
            fee_list: None,
        }
    }
}

