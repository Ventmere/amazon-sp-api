/*
 * Selling Partner API for Tokens 
 *
 * The Selling Partner API for Tokens provides a secure way to access a customer's PII (Personally Identifiable Information). You can call the Tokens API to get a Restricted Data Token (RDT) for one or more restricted resources that you specify. The RDT authorizes subsequent calls to restricted operations that correspond to the restricted resources that you specified.  For more information, see the [Tokens API Use Case Guide](doc:tokens-api-use-case-guide).
 *
 * The version of the OpenAPI document: 2021-03-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateRestrictedDataTokenResponse : The response schema for the createRestrictedDataToken operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateRestrictedDataTokenResponse {
    /// A Restricted Data Token (RDT). This is a short-lived access token that authorizes calls to restricted operations. Pass this value with the x-amz-access-token header when making subsequent calls to these restricted resources.
    #[serde(default, rename = "restrictedDataToken", skip_serializing_if = "Option::is_none")]
    pub restricted_data_token: Option<String>,
    /// The lifetime of the Restricted Data Token, in seconds.
    #[serde(default, rename = "expiresIn", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
}

impl CreateRestrictedDataTokenResponse {
    /// The response schema for the createRestrictedDataToken operation.
    pub fn new() -> CreateRestrictedDataTokenResponse {
        CreateRestrictedDataTokenResponse {
            restricted_data_token: None,
            expires_in: None,
        }
    }
}


