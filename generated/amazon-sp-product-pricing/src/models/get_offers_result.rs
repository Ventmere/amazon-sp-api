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
pub struct GetOffersResult {
    /// A marketplace identifier.
    #[serde(rename = "MarketplaceID")]
    pub marketplace_id: String,
    /// The Amazon Standard Identification Number (ASIN) of the item.
    #[serde(rename = "ASIN", skip_serializing_if = "Option::is_none")]
    pub ASIN: Option<String>,
    /// The stock keeping unit (SKU) of the item.
    #[serde(rename = "SKU", skip_serializing_if = "Option::is_none")]
    pub SKU: Option<String>,
    #[serde(rename = "ItemCondition")]
    pub item_condition: crate::models::ConditionType,
    /// The status of the operation.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "Identifier")]
    pub identifier: Box<crate::models::ItemIdentifier>,
    #[serde(rename = "Summary")]
    pub summary: Box<crate::models::Summary>,
    #[serde(rename = "Offers")]
    pub offers: Vec<crate::models::OfferDetail>,
}

impl GetOffersResult {
    pub fn new(marketplace_id: String, item_condition: crate::models::ConditionType, status: String, identifier: crate::models::ItemIdentifier, summary: crate::models::Summary, offers: Vec<crate::models::OfferDetail>) -> GetOffersResult {
        GetOffersResult {
            marketplace_id,
            ASIN: None,
            SKU: None,
            item_condition,
            status,
            identifier: Box::new(identifier),
            summary: Box::new(summary),
            offers,
        }
    }
}

