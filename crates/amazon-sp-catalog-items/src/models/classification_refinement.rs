/*
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items provides programmatic access to information about items in the Amazon catalog.  For more information, refer to the [Catalog Items API Use Case Guide](doc:catalog-items-api-v2022-04-01-use-case-guide).
 *
 * The version of the OpenAPI document: 2022-04-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ClassificationRefinement : Description of a classification that can be used to get more fine-grained search results.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClassificationRefinement {
    /// The estimated number of results that would still be returned if refinement key applied.
    #[serde(rename = "numberOfResults")]
    pub number_of_results: i32,
    /// Display name for the classification.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Identifier for the classification that can be used for search refinement purposes.
    #[serde(rename = "classificationId")]
    pub classification_id: String,
}

impl ClassificationRefinement {
    /// Description of a classification that can be used to get more fine-grained search results.
    pub fn new(number_of_results: i32, display_name: String, classification_id: String) -> ClassificationRefinement {
        ClassificationRefinement {
            number_of_results,
            display_name,
            classification_id,
        }
    }
}


