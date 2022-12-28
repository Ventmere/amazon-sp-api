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
pub struct ListingOffersRequest {
    /// The resource path of the operation you are calling in batch without any query parameters.  If you are calling `getItemOffersBatch`, supply the path of `getItemOffers`.  **Example:** `/products/pricing/v0/items/B000P6Q7MY/offers`  If you are calling `getListingOffersBatch`, supply the path of `getListingOffers`.  **Example:** `/products/pricing/v0/listings/B000P6Q7MY/offers`
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "method")]
    pub method: crate::models::HttpMethod,
    /// A mapping of additional HTTP headers to send/receive for the individual batch request.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    /// A marketplace identifier. Specifies the marketplace for which prices are returned.
    #[serde(rename = "MarketplaceId")]
    pub marketplace_id: String,
    #[serde(rename = "ItemCondition")]
    pub item_condition: crate::models::ItemCondition,
    #[serde(rename = "CustomerType", skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<crate::models::CustomerType>,
}

impl ListingOffersRequest {
    pub fn new(uri: String, method: crate::models::HttpMethod, marketplace_id: String, item_condition: crate::models::ItemCondition) -> ListingOffersRequest {
        ListingOffersRequest {
            uri,
            method,
            headers: None,
            marketplace_id,
            item_condition,
            customer_type: None,
        }
    }
}

