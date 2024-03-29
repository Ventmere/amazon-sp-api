/*
 * Selling Partner API for Pricing
 *
 * The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentifierType : Specifies the identifiers used to uniquely identify an item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentifierType {
    #[serde(default, rename = "MarketplaceASIN")]
    pub marketplace_asin: Box<crate::models::AsinIdentifier>,
    #[serde(default, rename = "SKUIdentifier", skip_serializing_if = "Option::is_none")]
    pub sku_identifier: Option<Box<crate::models::SellerSkuIdentifier>>,
}

impl IdentifierType {
    /// Specifies the identifiers used to uniquely identify an item.
    pub fn new(marketplace_asin: crate::models::AsinIdentifier) -> IdentifierType {
        IdentifierType {
            marketplace_asin: Box::new(marketplace_asin),
            sku_identifier: None,
        }
    }
}


