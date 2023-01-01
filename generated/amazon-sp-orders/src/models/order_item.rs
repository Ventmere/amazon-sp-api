/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderItem : A single order item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderItem {
    /// The Amazon Standard Identification Number (ASIN) of the item.
    #[serde(default, rename = "ASIN")]
    pub ASIN: String,
    /// The seller stock keeping unit (SKU) of the item.
    #[serde(default, rename = "SellerSKU", skip_serializing_if = "Option::is_none")]
    pub seller_sku: Option<String>,
    /// An Amazon-defined order item identifier.
    #[serde(default, rename = "OrderItemId")]
    pub order_item_id: String,
    /// The name of the item.
    #[serde(default, rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The number of items in the order. 
    #[serde(default, rename = "QuantityOrdered")]
    pub quantity_ordered: i32,
    /// The number of items shipped.
    #[serde(default, rename = "QuantityShipped", skip_serializing_if = "Option::is_none")]
    pub quantity_shipped: Option<i32>,
    #[serde(default, rename = "ProductInfo", skip_serializing_if = "Option::is_none")]
    pub product_info: Option<Box<crate::models::ProductInfoDetail>>,
    #[serde(default, rename = "PointsGranted", skip_serializing_if = "Option::is_none")]
    pub points_granted: Option<Box<crate::models::PointsGrantedDetail>>,
    #[serde(default, rename = "ItemPrice", skip_serializing_if = "Option::is_none")]
    pub item_price: Option<Box<crate::models::Money>>,
    #[serde(default, rename = "ShippingPrice", skip_serializing_if = "Option::is_none")]
    pub shipping_price: Option<Box<crate::models::Money>>,
    #[serde(default, rename = "ItemTax", skip_serializing_if = "Option::is_none")]
    pub item_tax: Option<Box<crate::models::Money>>,
    #[serde(default, rename = "ShippingTax", skip_serializing_if = "Option::is_none")]
    pub shipping_tax: Option<Box<crate::models::Money>>,
    #[serde(default, rename = "ShippingDiscount", skip_serializing_if = "Option::is_none")]
    pub shipping_discount: Option<Box<crate::models::Money>>,
    #[serde(default, rename = "ShippingDiscountTax", skip_serializing_if = "Option::is_none")]
    pub shipping_discount_tax: Option<Box<crate::models::Money>>,
    #[serde(default, rename = "PromotionDiscount", skip_serializing_if = "Option::is_none")]
    pub promotion_discount: Option<Box<crate::models::Money>>,
    #[serde(default, rename = "PromotionDiscountTax", skip_serializing_if = "Option::is_none")]
    pub promotion_discount_tax: Option<Box<crate::models::Money>>,
    /// A list of promotion identifiers provided by the seller when the promotions were created.
    #[serde(default, rename = "PromotionIds", skip_serializing_if = "Option::is_none")]
    pub promotion_ids: Option<Vec<String>>,
    #[serde(default, rename = "CODFee", skip_serializing_if = "Option::is_none")]
    pub cod_fee: Option<Box<crate::models::Money>>,
    #[serde(default, rename = "CODFeeDiscount", skip_serializing_if = "Option::is_none")]
    pub cod_fee_discount: Option<Box<crate::models::Money>>,
    /// When true, the item is a gift.
    #[serde(default, rename = "IsGift", skip_serializing_if = "Option::is_none", deserialize_with = "amazon_sp_api_shared::helpers::deserialize_opt_bool_from_string")]
    pub is_gift: Option<bool>,
    /// The condition of the item as described by the seller.
    #[serde(default, rename = "ConditionNote", skip_serializing_if = "Option::is_none")]
    pub condition_note: Option<String>,
    /// The condition of the item.  Possible values: New, Used, Collectible, Refurbished, Preorder, Club.
    #[serde(default, rename = "ConditionId", skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    /// The subcondition of the item.  Possible values: New, Mint, Very Good, Good, Acceptable, Poor, Club, OEM, Warranty, Refurbished Warranty, Refurbished, Open Box, Any, Other.
    #[serde(default, rename = "ConditionSubtypeId", skip_serializing_if = "Option::is_none")]
    pub condition_subtype_id: Option<String>,
    /// The start date of the scheduled delivery window in the time zone of the order destination. In ISO 8601 date time format.
    #[serde(default, rename = "ScheduledDeliveryStartDate", skip_serializing_if = "Option::is_none")]
    pub scheduled_delivery_start_date: Option<String>,
    /// The end date of the scheduled delivery window in the time zone of the order destination. In ISO 8601 date time format.
    #[serde(default, rename = "ScheduledDeliveryEndDate", skip_serializing_if = "Option::is_none")]
    pub scheduled_delivery_end_date: Option<String>,
    /// Indicates that the selling price is a special price that is available only for Amazon Business orders. For more information about the Amazon Business Seller Program, see the [Amazon Business website](https://www.amazon.com/b2b/info/amazon-business).   Possible values: BusinessPrice - A special price that is available only for Amazon Business orders.
    #[serde(default, rename = "PriceDesignation", skip_serializing_if = "Option::is_none")]
    pub price_designation: Option<String>,
    #[serde(default, rename = "TaxCollection", skip_serializing_if = "Option::is_none")]
    pub tax_collection: Option<Box<crate::models::TaxCollection>>,
    /// When true, the product type for this item has a serial number.  Returned only for Amazon Easy Ship orders.
    #[serde(default, rename = "SerialNumberRequired", skip_serializing_if = "Option::is_none")]
    pub serial_number_required: Option<bool>,
    /// When true, transparency codes are required.
    #[serde(default, rename = "IsTransparency", skip_serializing_if = "Option::is_none")]
    pub is_transparency: Option<bool>,
    /// The IOSS number for the marketplace. Sellers shipping to the European Union (EU) from outside of the EU must provide this IOSS number to their carrier when Amazon has collected the VAT on the sale.
    #[serde(default, rename = "IossNumber", skip_serializing_if = "Option::is_none")]
    pub ioss_number: Option<String>,
    /// The store chain store identifier. Linked to a specific store in a store chain.
    #[serde(default, rename = "StoreChainStoreId", skip_serializing_if = "Option::is_none")]
    pub store_chain_store_id: Option<String>,
    /// The category of deemed reseller. This applies to selling partners that are not based in the EU and is used to help them meet the VAT Deemed Reseller tax laws in the EU and UK.
    #[serde(default, rename = "DeemedResellerCategory", skip_serializing_if = "Option::is_none")]
    pub deemed_reseller_category: Option<DeemedResellerCategory>,
    #[serde(default, rename = "BuyerInfo", skip_serializing_if = "Option::is_none")]
    pub buyer_info: Option<Box<crate::models::ItemBuyerInfo>>,
    #[serde(default, rename = "BuyerRequestedCancel", skip_serializing_if = "Option::is_none")]
    pub buyer_requested_cancel: Option<Box<crate::models::BuyerRequestedCancel>>,
}

impl OrderItem {
    /// A single order item.
    pub fn new(ASIN: String, order_item_id: String, quantity_ordered: i32) -> OrderItem {
        OrderItem {
            ASIN,
            seller_sku: None,
            order_item_id,
            title: None,
            quantity_ordered,
            quantity_shipped: None,
            product_info: None,
            points_granted: None,
            item_price: None,
            shipping_price: None,
            item_tax: None,
            shipping_tax: None,
            shipping_discount: None,
            shipping_discount_tax: None,
            promotion_discount: None,
            promotion_discount_tax: None,
            promotion_ids: None,
            cod_fee: None,
            cod_fee_discount: None,
            is_gift: None,
            condition_note: None,
            condition_id: None,
            condition_subtype_id: None,
            scheduled_delivery_start_date: None,
            scheduled_delivery_end_date: None,
            price_designation: None,
            tax_collection: None,
            serial_number_required: None,
            is_transparency: None,
            ioss_number: None,
            store_chain_store_id: None,
            deemed_reseller_category: None,
            buyer_info: None,
            buyer_requested_cancel: None,
        }
    }
}

/// The category of deemed reseller. This applies to selling partners that are not based in the EU and is used to help them meet the VAT Deemed Reseller tax laws in the EU and UK.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeemedResellerCategory {
    #[serde(rename = "IOSS")]
    IOSS,
    #[serde(rename = "UOSS")]
    UOSS,
}

impl Default for DeemedResellerCategory {
    fn default() -> DeemedResellerCategory {
        Self::IOSS
    }
}

