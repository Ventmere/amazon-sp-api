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
pub struct ListingOffersRequestParamsAllOf {
    /// The seller stock keeping unit (SKU) of the item. This is the same SKU passed as a path parameter.
    #[serde(default, rename = "SellerSKU")]
    pub seller_sku: String,
}

impl ListingOffersRequestParamsAllOf {
    pub fn new(seller_sku: String) -> ListingOffersRequestParamsAllOf {
        ListingOffersRequestParamsAllOf {
            seller_sku,
        }
    }
}


