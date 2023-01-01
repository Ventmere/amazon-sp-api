/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BuyerCustomizedInfoDetail : Buyer information for custom orders from the Amazon Custom program.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BuyerCustomizedInfoDetail {
    /// The location of a zip file containing Amazon Custom data.
    #[serde(default, rename = "CustomizedURL", skip_serializing_if = "Option::is_none")]
    pub customized_url: Option<String>,
}

impl BuyerCustomizedInfoDetail {
    /// Buyer information for custom orders from the Amazon Custom program.
    pub fn new() -> BuyerCustomizedInfoDetail {
        BuyerCustomizedInfoDetail {
            customized_url: None,
        }
    }
}


