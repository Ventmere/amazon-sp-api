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
pub struct SalesRankType {
    ///  Identifies the item category from which the sales rank is taken.
    #[serde(rename = "ProductCategoryId")]
    pub product_category_id: String,
    /// The sales rank of the item within the item category.
    #[serde(rename = "Rank")]
    pub rank: i32,
}

impl SalesRankType {
    pub fn new(product_category_id: String, rank: i32) -> SalesRankType {
        SalesRankType {
            product_category_id,
            rank,
        }
    }
}

