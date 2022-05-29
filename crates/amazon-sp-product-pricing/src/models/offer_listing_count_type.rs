/*
 * Selling Partner API for Pricing
 *
 * The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OfferListingCountType : The number of offer listings with the specified condition.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferListingCountType {
    /// The number of offer listings.
    #[serde(rename = "Count")]
    pub count: i32,
    /// The condition of the item.
    #[serde(rename = "condition")]
    pub condition: String,
}

impl OfferListingCountType {
    /// The number of offer listings with the specified condition.
    pub fn new(count: i32, condition: String) -> OfferListingCountType {
        OfferListingCountType {
            count,
            condition,
        }
    }
}


