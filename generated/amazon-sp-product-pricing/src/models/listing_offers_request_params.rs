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
pub struct ListingOffersRequestParams {
    /// A marketplace identifier. Specifies the marketplace for which prices are returned.
    #[serde(default, rename = "MarketplaceId")]
    pub marketplace_id: String,
    #[serde(default, rename = "ItemCondition")]
    pub item_condition: crate::models::ItemCondition,
    #[serde(default, rename = "CustomerType", skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<crate::models::CustomerType>,
    /// The seller stock keeping unit (SKU) of the item. This is the same SKU passed as a path parameter.
    #[serde(default, rename = "SellerSKU")]
    pub seller_sku: String,
}

impl ListingOffersRequestParams {
    pub fn new(marketplace_id: String, item_condition: crate::models::ItemCondition, seller_sku: String) -> ListingOffersRequestParams {
        ListingOffersRequestParams {
            marketplace_id,
            item_condition,
            customer_type: None,
            seller_sku,
        }
    }
}


