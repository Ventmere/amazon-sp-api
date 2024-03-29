/*
 * Selling Partner API for Product Fees
 *
 * The Selling Partner API for Product Fees lets you programmatically retrieve estimated fees for a product. You can then account for those fees in your pricing.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdType : The type of product identifier used in a `FeesEstimateByIdRequest`.

/// The type of product identifier used in a `FeesEstimateByIdRequest`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdType {
    #[serde(rename = "ASIN")]
    ASIN,
    #[serde(rename = "SellerSKU")]
    SellerSKU,

}

impl ToString for IdType {
    fn to_string(&self) -> String {
        match self {
            Self::ASIN => String::from("ASIN"),
            Self::SellerSKU => String::from("SellerSKU"),
        }
    }
}

impl Default for IdType {
    fn default() -> IdType {
        Self::ASIN
    }
}




