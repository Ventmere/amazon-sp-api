/*
 * Selling Partner API for Product Fees
 *
 * The Selling Partner API for Product Fees lets you programmatically retrieve estimated fees for a product. You can then account for those fees in your pricing.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OptionalFulfillmentProgram : An optional enrollment program to return the estimated fees when the offer is fulfilled by Amazon (IsAmazonFulfilled is set to true).

/// An optional enrollment program to return the estimated fees when the offer is fulfilled by Amazon (IsAmazonFulfilled is set to true).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OptionalFulfillmentProgram {
    #[serde(rename = "FBA_CORE")]
    CORE,
    #[serde(rename = "FBA_SNL")]
    SNL,
    #[serde(rename = "FBA_EFN")]
    EFN,

}

impl ToString for OptionalFulfillmentProgram {
    fn to_string(&self) -> String {
        match self {
            Self::CORE => String::from("FBA_CORE"),
            Self::SNL => String::from("FBA_SNL"),
            Self::EFN => String::from("FBA_EFN"),
        }
    }
}

impl Default for OptionalFulfillmentProgram {
    fn default() -> OptionalFulfillmentProgram {
        Self::CORE
    }
}



