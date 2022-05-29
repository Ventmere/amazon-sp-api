/*
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items provides programmatic access to information about items in the Amazon catalog.  For more information, refer to the [Catalog Items API Use Case Guide](doc:catalog-items-api-v2022-04-01-use-case-guide).
 *
 * The version of the OpenAPI document: 2022-04-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BrandRefinement : Description of a brand that can be used to get more fine-grained search results.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BrandRefinement {
    /// The estimated number of results that would still be returned if refinement key applied.
    #[serde(rename = "numberOfResults")]
    pub number_of_results: i32,
    /// Brand name. For display and can be used as a search refinement.
    #[serde(rename = "brandName")]
    pub brand_name: String,
}

impl BrandRefinement {
    /// Description of a brand that can be used to get more fine-grained search results.
    pub fn new(number_of_results: i32, brand_name: String) -> BrandRefinement {
        BrandRefinement {
            number_of_results,
            brand_name,
        }
    }
}


