/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UnitOfWeight : The unit of weight.

/// The unit of weight.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnitOfWeight {
    #[serde(rename = "oz")]
    Oz,
    #[serde(rename = "g")]
    G,

}

impl ToString for UnitOfWeight {
    fn to_string(&self) -> String {
        match self {
            Self::Oz => String::from("oz"),
            Self::G => String::from("g"),
        }
    }
}

impl Default for UnitOfWeight {
    fn default() -> UnitOfWeight {
        Self::Oz
    }
}




