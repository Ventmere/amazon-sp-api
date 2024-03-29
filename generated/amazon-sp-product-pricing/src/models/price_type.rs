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
pub struct PriceType {
    #[serde(default, rename = "LandedPrice", skip_serializing_if = "Option::is_none")]
    pub landed_price: Option<Box<crate::models::MoneyType>>,
    #[serde(default, rename = "ListingPrice")]
    pub listing_price: Box<crate::models::MoneyType>,
    #[serde(default, rename = "Shipping", skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Box<crate::models::MoneyType>>,
    #[serde(default, rename = "Points", skip_serializing_if = "Option::is_none")]
    pub points: Option<Box<crate::models::Points>>,
}

impl PriceType {
    pub fn new(listing_price: crate::models::MoneyType) -> PriceType {
        PriceType {
            landed_price: None,
            listing_price: Box::new(listing_price),
            shipping: None,
            points: None,
        }
    }
}


