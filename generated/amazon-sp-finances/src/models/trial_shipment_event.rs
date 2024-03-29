/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TrialShipmentEvent : An event related to a trial shipment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TrialShipmentEvent {
    /// An Amazon-defined identifier for an order.
    #[serde(default, rename = "AmazonOrderId", skip_serializing_if = "Option::is_none")]
    pub amazon_order_id: Option<String>,
    /// The identifier of the financial event group.
    #[serde(default, rename = "FinancialEventGroupId", skip_serializing_if = "Option::is_none")]
    pub financial_event_group_id: Option<String>,
    #[serde(default, rename = "PostedDate", skip_serializing_if = "Option::is_none")]
    pub posted_date: Option<String>,
    /// The seller SKU of the item. The seller SKU is qualified by the seller's seller ID, which is included with every call to the Selling Partner API.
    #[serde(default, rename = "SKU", skip_serializing_if = "Option::is_none")]
    pub SKU: Option<String>,
    /// A list of fee component information.
    #[serde(default, rename = "FeeList", skip_serializing_if = "Option::is_none")]
    pub fee_list: Option<Vec<crate::models::FeeComponent>>,
}

impl TrialShipmentEvent {
    /// An event related to a trial shipment.
    pub fn new() -> TrialShipmentEvent {
        TrialShipmentEvent {
            amazon_order_id: None,
            financial_event_group_id: None,
            posted_date: None,
            SKU: None,
            fee_list: None,
        }
    }
}


