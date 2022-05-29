/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemLevelFields {
    /// The Amazon Standard Identification Number (ASIN) of the item.
    #[serde(rename = "Asin")]
    pub asin: String,
    /// A list of additional inputs.
    #[serde(rename = "AdditionalInputs")]
    pub additional_inputs: Vec<crate::models::AdditionalInputs>,
}

impl ItemLevelFields {
    pub fn new(asin: String, additional_inputs: Vec<crate::models::AdditionalInputs>) -> ItemLevelFields {
        ItemLevelFields {
            asin,
            additional_inputs,
        }
    }
}


