/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServiceFeeEvent : A service fee on the seller's account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceFeeEvent {
    /// An Amazon-defined identifier for an order.
    #[serde(default, rename = "AmazonOrderId", skip_serializing_if = "Option::is_none")]
    pub amazon_order_id: Option<String>,
    /// A short description of the service fee reason.
    #[serde(default, rename = "FeeReason", skip_serializing_if = "Option::is_none")]
    pub fee_reason: Option<String>,
    /// A list of fee component information.
    #[serde(default, rename = "FeeList", skip_serializing_if = "Option::is_none")]
    pub fee_list: Option<Vec<crate::models::FeeComponent>>,
    /// The seller SKU of the item. The seller SKU is qualified by the seller's seller ID, which is included with every call to the Selling Partner API.
    #[serde(default, rename = "SellerSKU", skip_serializing_if = "Option::is_none")]
    pub seller_sku: Option<String>,
    /// A unique identifier assigned by Amazon to products stored in and fulfilled from an Amazon fulfillment center.
    #[serde(default, rename = "FnSKU", skip_serializing_if = "Option::is_none")]
    pub fn_sku: Option<String>,
    /// A short description of the service fee event.
    #[serde(default, rename = "FeeDescription", skip_serializing_if = "Option::is_none")]
    pub fee_description: Option<String>,
    /// The Amazon Standard Identification Number (ASIN) of the item.
    #[serde(default, rename = "ASIN", skip_serializing_if = "Option::is_none")]
    pub ASIN: Option<String>,
}

impl ServiceFeeEvent {
    /// A service fee on the seller's account.
    pub fn new() -> ServiceFeeEvent {
        ServiceFeeEvent {
            amazon_order_id: None,
            fee_reason: None,
            fee_list: None,
            seller_sku: None,
            fn_sku: None,
            fee_description: None,
            ASIN: None,
        }
    }
}


