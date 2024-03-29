/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentItem : An item of a shipment, refund, guarantee claim, or chargeback.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipmentItem {
    /// The seller SKU of the item. The seller SKU is qualified by the seller's seller ID, which is included with every call to the Selling Partner API.
    #[serde(default, rename = "SellerSKU", skip_serializing_if = "Option::is_none")]
    pub seller_sku: Option<String>,
    /// An Amazon-defined order item identifier.
    #[serde(default, rename = "OrderItemId", skip_serializing_if = "Option::is_none")]
    pub order_item_id: Option<String>,
    /// An Amazon-defined order adjustment identifier defined for refunds, guarantee claims, and chargeback events.
    #[serde(default, rename = "OrderAdjustmentItemId", skip_serializing_if = "Option::is_none")]
    pub order_adjustment_item_id: Option<String>,
    /// The number of items shipped.
    #[serde(default, rename = "QuantityShipped", skip_serializing_if = "Option::is_none")]
    pub quantity_shipped: Option<i32>,
    /// A list of charge information on the seller's account.
    #[serde(default, rename = "ItemChargeList", skip_serializing_if = "Option::is_none")]
    pub item_charge_list: Option<Vec<crate::models::ChargeComponent>>,
    /// A list of charge information on the seller's account.
    #[serde(default, rename = "ItemChargeAdjustmentList", skip_serializing_if = "Option::is_none")]
    pub item_charge_adjustment_list: Option<Vec<crate::models::ChargeComponent>>,
    /// A list of fee component information.
    #[serde(default, rename = "ItemFeeList", skip_serializing_if = "Option::is_none")]
    pub item_fee_list: Option<Vec<crate::models::FeeComponent>>,
    /// A list of fee component information.
    #[serde(default, rename = "ItemFeeAdjustmentList", skip_serializing_if = "Option::is_none")]
    pub item_fee_adjustment_list: Option<Vec<crate::models::FeeComponent>>,
    /// A list of information about taxes withheld.
    #[serde(default, rename = "ItemTaxWithheldList", skip_serializing_if = "Option::is_none")]
    pub item_tax_withheld_list: Option<Vec<crate::models::TaxWithheldComponent>>,
    /// A list of promotions.
    #[serde(default, rename = "PromotionList", skip_serializing_if = "Option::is_none")]
    pub promotion_list: Option<Vec<crate::models::Promotion>>,
    /// A list of promotions.
    #[serde(default, rename = "PromotionAdjustmentList", skip_serializing_if = "Option::is_none")]
    pub promotion_adjustment_list: Option<Vec<crate::models::Promotion>>,
    #[serde(default, rename = "CostOfPointsGranted", skip_serializing_if = "Option::is_none")]
    pub cost_of_points_granted: Option<Box<crate::models::Currency>>,
    #[serde(default, rename = "CostOfPointsReturned", skip_serializing_if = "Option::is_none")]
    pub cost_of_points_returned: Option<Box<crate::models::Currency>>,
}

impl ShipmentItem {
    /// An item of a shipment, refund, guarantee claim, or chargeback.
    pub fn new() -> ShipmentItem {
        ShipmentItem {
            seller_sku: None,
            order_item_id: None,
            order_adjustment_item_id: None,
            quantity_shipped: None,
            item_charge_list: None,
            item_charge_adjustment_list: None,
            item_fee_list: None,
            item_fee_adjustment_list: None,
            item_tax_withheld_list: None,
            promotion_list: None,
            promotion_adjustment_list: None,
            cost_of_points_granted: None,
            cost_of_points_returned: None,
        }
    }
}


