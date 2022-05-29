/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CouponPaymentEvent : An event related to coupon payments.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CouponPaymentEvent {
    #[serde(rename = "PostedDate", skip_serializing_if = "Option::is_none")]
    pub posted_date: Option<String>,
    /// A coupon identifier.
    #[serde(rename = "CouponId", skip_serializing_if = "Option::is_none")]
    pub coupon_id: Option<String>,
    /// The description provided by the seller when they created the coupon.
    #[serde(rename = "SellerCouponDescription", skip_serializing_if = "Option::is_none")]
    pub seller_coupon_description: Option<String>,
    /// The number of coupon clips or redemptions.
    #[serde(rename = "ClipOrRedemptionCount", skip_serializing_if = "Option::is_none")]
    pub clip_or_redemption_count: Option<i64>,
    /// A payment event identifier.
    #[serde(rename = "PaymentEventId", skip_serializing_if = "Option::is_none")]
    pub payment_event_id: Option<String>,
    #[serde(rename = "FeeComponent", skip_serializing_if = "Option::is_none")]
    pub fee_component: Option<Box<crate::models::FeeComponent>>,
    #[serde(rename = "ChargeComponent", skip_serializing_if = "Option::is_none")]
    pub charge_component: Option<Box<crate::models::ChargeComponent>>,
    #[serde(rename = "TotalAmount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<Box<crate::models::Currency>>,
}

impl CouponPaymentEvent {
    /// An event related to coupon payments.
    pub fn new() -> CouponPaymentEvent {
        CouponPaymentEvent {
            posted_date: None,
            coupon_id: None,
            seller_coupon_description: None,
            clip_or_redemption_count: None,
            payment_event_id: None,
            fee_component: None,
            charge_component: None,
            total_amount: None,
        }
    }
}


