/*
 * Selling Partner API for Product Type Definitions
 *
 * The Selling Partner API for Product Type Definitions provides programmatic access to attribute and data requirements for product types in the Amazon catalog. Use this API to return the JSON Schema for a product type that you can then use with other Selling Partner APIs, such as the Selling Partner API for Listings Items, the Selling Partner API for Catalog Items, and the Selling Partner API for Feeds (for JSON-based listing feeds).  For more information, see the [Product Type Definitions API Use Case Guide](doc:product-type-api-use-case-guide).
 *
 * The version of the OpenAPI document: 2020-09-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ErrorList : A list of error responses returned when a request is unsuccessful.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorList {
    #[serde(default, rename = "errors")]
    pub errors: Vec<crate::models::Error>,
}

impl ErrorList {
    /// A list of error responses returned when a request is unsuccessful.
    pub fn new(errors: Vec<crate::models::Error>) -> ErrorList {
        ErrorList {
            errors,
        }
    }
}


