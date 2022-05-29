/*
 * Selling Partner API for Listings Items
 *
 * The Selling Partner API for Listings Items (Listings Items API) provides programmatic access to selling partner listings on Amazon. Use this API in collaboration with the Selling Partner API for Product Type Definitions, which you use to retrieve the information about Amazon product types needed to use the Listings Items API.  For more information, see the [Listings Items API Use Case Guide](doc:listings-items-api-v2021-08-01-use-case-guide).
 *
 * The version of the OpenAPI document: 2021-08-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListingsItemPatchRequest : The request body schema for the patchListingsItem operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListingsItemPatchRequest {
    /// The Amazon product type of the listings item.
    #[serde(rename = "productType")]
    pub product_type: String,
    /// One or more JSON Patch operations to perform on the listings item.
    #[serde(rename = "patches")]
    pub patches: Vec<crate::models::PatchOperation>,
}

impl ListingsItemPatchRequest {
    /// The request body schema for the patchListingsItem operation.
    pub fn new(product_type: String, patches: Vec<crate::models::PatchOperation>) -> ListingsItemPatchRequest {
        ListingsItemPatchRequest {
            product_type,
            patches,
        }
    }
}


