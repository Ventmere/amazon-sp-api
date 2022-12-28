/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SolutionProviderCreditEvent : A credit given to a solution provider.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SolutionProviderCreditEvent {
    /// The transaction type.
    #[serde(rename = "ProviderTransactionType", skip_serializing_if = "Option::is_none")]
    pub provider_transaction_type: Option<String>,
    /// A seller-defined identifier for an order.
    #[serde(rename = "SellerOrderId", skip_serializing_if = "Option::is_none")]
    pub seller_order_id: Option<String>,
    /// The identifier of the marketplace where the order was placed.
    #[serde(rename = "MarketplaceId", skip_serializing_if = "Option::is_none")]
    pub marketplace_id: Option<String>,
    /// The two-letter country code of the country associated with the marketplace where the order was placed.
    #[serde(rename = "MarketplaceCountryCode", skip_serializing_if = "Option::is_none")]
    pub marketplace_country_code: Option<String>,
    /// The Amazon-defined identifier of the seller.
    #[serde(rename = "SellerId", skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
    /// The store name where the payment event occurred.
    #[serde(rename = "SellerStoreName", skip_serializing_if = "Option::is_none")]
    pub seller_store_name: Option<String>,
    /// The Amazon-defined identifier of the solution provider.
    #[serde(rename = "ProviderId", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    /// The store name where the payment event occurred.
    #[serde(rename = "ProviderStoreName", skip_serializing_if = "Option::is_none")]
    pub provider_store_name: Option<String>,
    #[serde(rename = "TransactionAmount", skip_serializing_if = "Option::is_none")]
    pub transaction_amount: Option<Box<crate::models::Currency>>,
    #[serde(rename = "TransactionCreationDate", skip_serializing_if = "Option::is_none")]
    pub transaction_creation_date: Option<String>,
}

impl SolutionProviderCreditEvent {
    /// A credit given to a solution provider.
    pub fn new() -> SolutionProviderCreditEvent {
        SolutionProviderCreditEvent {
            provider_transaction_type: None,
            seller_order_id: None,
            marketplace_id: None,
            marketplace_country_code: None,
            seller_id: None,
            seller_store_name: None,
            provider_id: None,
            provider_store_name: None,
            transaction_amount: None,
            transaction_creation_date: None,
        }
    }
}

