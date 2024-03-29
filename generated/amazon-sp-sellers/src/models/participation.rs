/*
 * Selling Partner API for Sellers
 *
 * The Selling Partner API for Sellers lets you retrieve information on behalf of sellers about their seller account, such as the marketplaces they participate in. Along with listing the marketplaces that a seller can sell in, the API also provides additional information about the marketplace such as the default language and the default currency. The API also provides seller-specific information such as whether the seller has suspended listings in that marketplace.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Participation : Detailed information that is specific to a seller in a Marketplace.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Participation {
    #[serde(default, rename = "isParticipating")]
    pub is_participating: bool,
    /// Specifies if the seller has suspended listings. True if the seller Listing Status is set to Inactive, otherwise False.
    #[serde(default, rename = "hasSuspendedListings")]
    pub has_suspended_listings: bool,
}

impl Participation {
    /// Detailed information that is specific to a seller in a Marketplace.
    pub fn new(is_participating: bool, has_suspended_listings: bool) -> Participation {
        Participation {
            is_participating,
            has_suspended_listings,
        }
    }
}


