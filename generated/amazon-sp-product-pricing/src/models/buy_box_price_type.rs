/*
 * Selling Partner API for Pricing
 *
 * The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BuyBoxPriceType {
    /// Indicates the condition of the item. For example: New, Used, Collectible, Refurbished, or Club.
    #[serde(default, rename = "condition")]
    pub condition: String,
    #[serde(default, rename = "offerType", skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<crate::models::OfferCustomerType>,
    /// Indicates at what quantity this price becomes active.
    #[serde(default, rename = "quantityTier", skip_serializing_if = "Option::is_none")]
    pub quantity_tier: Option<i32>,
    #[serde(default, rename = "quantityDiscountType", skip_serializing_if = "Option::is_none")]
    pub quantity_discount_type: Option<crate::models::QuantityDiscountType>,
    #[serde(default, rename = "LandedPrice")]
    pub landed_price: Box<crate::models::MoneyType>,
    #[serde(default, rename = "ListingPrice")]
    pub listing_price: Box<crate::models::MoneyType>,
    #[serde(default, rename = "Shipping")]
    pub shipping: Box<crate::models::MoneyType>,
    #[serde(default, rename = "Points", skip_serializing_if = "Option::is_none")]
    pub points: Option<Box<crate::models::Points>>,
    /// The seller identifier for the offer.
    #[serde(default, rename = "sellerId", skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
}

impl BuyBoxPriceType {
    pub fn new(condition: String, landed_price: crate::models::MoneyType, listing_price: crate::models::MoneyType, shipping: crate::models::MoneyType) -> BuyBoxPriceType {
        BuyBoxPriceType {
            condition,
            offer_type: None,
            quantity_tier: None,
            quantity_discount_type: None,
            landed_price: Box::new(landed_price),
            listing_price: Box::new(listing_price),
            shipping: Box::new(shipping),
            points: None,
            seller_id: None,
        }
    }
}


