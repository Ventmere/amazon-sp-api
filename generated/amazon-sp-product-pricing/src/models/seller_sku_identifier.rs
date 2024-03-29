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
pub struct SellerSkuIdentifier {
    /// A marketplace identifier.
    #[serde(default, rename = "MarketplaceId")]
    pub marketplace_id: String,
    /// The seller identifier submitted for the operation.
    #[serde(default, rename = "SellerId")]
    pub seller_id: String,
    /// The seller stock keeping unit (SKU) of the item.
    #[serde(default, rename = "SellerSKU")]
    pub seller_sku: String,
}

impl SellerSkuIdentifier {
    pub fn new(marketplace_id: String, seller_id: String, seller_sku: String) -> SellerSkuIdentifier {
        SellerSkuIdentifier {
            marketplace_id,
            seller_id,
            seller_sku,
        }
    }
}


