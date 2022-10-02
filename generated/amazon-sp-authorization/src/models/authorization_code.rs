/*
 * Selling Partner API for Authorization
 *
 * The Selling Partner API for Authorization helps developers manage authorizations and check the specific permissions associated with a given authorization.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthorizationCode : A Login with Amazon (LWA) authorization code.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AuthorizationCode {
    /// A Login with Amazon (LWA) authorization code that can be exchanged for a refresh token and access token that authorize you to make calls to a Selling Partner API.
    #[serde(rename = "authorizationCode", skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
}

impl AuthorizationCode {
    /// A Login with Amazon (LWA) authorization code.
    pub fn new() -> AuthorizationCode {
        AuthorizationCode {
            authorization_code: None,
        }
    }
}


