/*
 * Selling Partner API for Listings Restrictions
 *
 * The Selling Partner API for Listings Restrictions provides programmatic access to restrictions on Amazon catalog listings.  For more information, see the [Listings Restrictions API Use Case Guide](doc:listings-restrictions-api-v2021-08-01-use-case-guide).
 *
 * The version of the OpenAPI document: 2021-08-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RestrictionList : A list of restrictions for the specified Amazon catalog item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RestrictionList {
    #[serde(rename = "restrictions")]
    pub restrictions: Vec<crate::models::Restriction>,
}

impl RestrictionList {
    /// A list of restrictions for the specified Amazon catalog item.
    pub fn new(restrictions: Vec<crate::models::Restriction>) -> RestrictionList {
        RestrictionList {
            restrictions,
        }
    }
}

