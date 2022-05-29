/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RentalTransactionEvent : An event related to a rental transaction.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RentalTransactionEvent {
    /// An Amazon-defined identifier for an order.
    #[serde(rename = "AmazonOrderId", skip_serializing_if = "Option::is_none")]
    pub amazon_order_id: Option<String>,
    /// The type of rental event.  Possible values:  * RentalCustomerPayment-Buyout - Transaction type that represents when the customer wants to buy out a rented item.  * RentalCustomerPayment-Extension - Transaction type that represents when the customer wants to extend the rental period.  * RentalCustomerRefund-Buyout - Transaction type that represents when the customer requests a refund for the buyout of the rented item.  * RentalCustomerRefund-Extension - Transaction type that represents when the customer requests a refund over the extension on the rented item.  * RentalHandlingFee - Transaction type that represents the fee that Amazon charges sellers who rent through Amazon.  * RentalChargeFailureReimbursement - Transaction type that represents when Amazon sends money to the seller to compensate for a failed charge.  * RentalLostItemReimbursement - Transaction type that represents when Amazon sends money to the seller to compensate for a lost item.
    #[serde(rename = "RentalEventType", skip_serializing_if = "Option::is_none")]
    pub rental_event_type: Option<String>,
    /// The number of days that the buyer extended an already rented item. This value is only returned for RentalCustomerPayment-Extension and RentalCustomerRefund-Extension events.
    #[serde(rename = "ExtensionLength", skip_serializing_if = "Option::is_none")]
    pub extension_length: Option<i32>,
    #[serde(rename = "PostedDate", skip_serializing_if = "Option::is_none")]
    pub posted_date: Option<String>,
    /// A list of charge information on the seller's account.
    #[serde(rename = "RentalChargeList", skip_serializing_if = "Option::is_none")]
    pub rental_charge_list: Option<Vec<crate::models::ChargeComponent>>,
    /// A list of fee component information.
    #[serde(rename = "RentalFeeList", skip_serializing_if = "Option::is_none")]
    pub rental_fee_list: Option<Vec<crate::models::FeeComponent>>,
    /// The name of the marketplace.
    #[serde(rename = "MarketplaceName", skip_serializing_if = "Option::is_none")]
    pub marketplace_name: Option<String>,
    #[serde(rename = "RentalInitialValue", skip_serializing_if = "Option::is_none")]
    pub rental_initial_value: Option<Box<crate::models::Currency>>,
    #[serde(rename = "RentalReimbursement", skip_serializing_if = "Option::is_none")]
    pub rental_reimbursement: Option<Box<crate::models::Currency>>,
    /// A list of information about taxes withheld.
    #[serde(rename = "RentalTaxWithheldList", skip_serializing_if = "Option::is_none")]
    pub rental_tax_withheld_list: Option<Vec<crate::models::TaxWithheldComponent>>,
}

impl RentalTransactionEvent {
    /// An event related to a rental transaction.
    pub fn new() -> RentalTransactionEvent {
        RentalTransactionEvent {
            amazon_order_id: None,
            rental_event_type: None,
            extension_length: None,
            posted_date: None,
            rental_charge_list: None,
            rental_fee_list: None,
            marketplace_name: None,
            rental_initial_value: None,
            rental_reimbursement: None,
            rental_tax_withheld_list: None,
        }
    }
}


