/*
 * Selling Partner API for Product Fees
 *
 * The Selling Partner API for Product Fees lets you programmatically retrieve estimated fees for a product. You can then account for those fees in your pricing.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeesEstimateRequest {
    /// A marketplace identifier.
    #[serde(default, rename = "MarketplaceId")]
    pub marketplace_id: String,
    /// When true, the offer is fulfilled by Amazon.
    #[serde(default, rename = "IsAmazonFulfilled", skip_serializing_if = "Option::is_none")]
    pub is_amazon_fulfilled: Option<bool>,
    #[serde(default, rename = "PriceToEstimateFees")]
    pub price_to_estimate_fees: Box<crate::models::PriceToEstimateFees>,
    /// A unique identifier provided by the caller to track this request.
    #[serde(default, rename = "Identifier")]
    pub identifier: String,
    #[serde(default, rename = "OptionalFulfillmentProgram", skip_serializing_if = "Option::is_none")]
    pub optional_fulfillment_program: Option<crate::models::OptionalFulfillmentProgram>,
}

impl FeesEstimateRequest {
    pub fn new(marketplace_id: String, price_to_estimate_fees: crate::models::PriceToEstimateFees, identifier: String) -> FeesEstimateRequest {
        FeesEstimateRequest {
            marketplace_id,
            is_amazon_fulfilled: None,
            price_to_estimate_fees: Box::new(price_to_estimate_fees),
            identifier,
            optional_fulfillment_program: None,
        }
    }
}


