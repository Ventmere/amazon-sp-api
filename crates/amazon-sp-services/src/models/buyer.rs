/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Buyer : Information about the buyer.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Buyer {
    /// The identifier of the buyer.
    #[serde(rename = "buyerId", skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    /// The name of the buyer.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The phone number of the buyer.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// When true, the service is for an Amazon Prime buyer.
    #[serde(rename = "isPrimeMember", skip_serializing_if = "Option::is_none")]
    pub is_prime_member: Option<bool>,
}

impl Buyer {
    /// Information about the buyer.
    pub fn new() -> Buyer {
        Buyer {
            buyer_id: None,
            name: None,
            phone: None,
            is_prime_member: None,
        }
    }
}


