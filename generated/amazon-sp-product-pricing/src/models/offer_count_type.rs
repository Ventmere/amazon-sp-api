/*
 * Selling Partner API for Pricing
 *
 * The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OfferCountType : The total number of offers for the specified condition and fulfillment channel.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OfferCountType {
    /// Indicates the condition of the item. For example: New, Used, Collectible, Refurbished, or Club.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "fulfillmentChannel", skip_serializing_if = "Option::is_none")]
    pub fulfillment_channel: Option<crate::models::FulfillmentChannelType>,
    /// The number of offers in a fulfillment channel that meet a specific condition.
    #[serde(rename = "OfferCount", skip_serializing_if = "Option::is_none")]
    pub offer_count: Option<i32>,
}

impl OfferCountType {
    /// The total number of offers for the specified condition and fulfillment channel.
    pub fn new() -> OfferCountType {
        OfferCountType {
            condition: None,
            fulfillment_channel: None,
            offer_count: None,
        }
    }
}

