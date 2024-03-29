/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Fee : Fee type and cost.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Fee {
    /// The type of fee.
    #[serde(default, rename = "name")]
    pub name: Name,
    #[serde(default, rename = "amount")]
    pub amount: Box<crate::models::Money>,
}

impl Fee {
    /// Fee type and cost.
    pub fn new(name: Name, amount: crate::models::Money) -> Fee {
        Fee {
            name,
            amount: Box::new(amount),
        }
    }
}

/// The type of fee.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "FBAPerUnitFulfillmentFee")]
    FBAPerUnitFulfillmentFee,
    #[serde(rename = "FBAPerOrderFulfillmentFee")]
    FBAPerOrderFulfillmentFee,
    #[serde(rename = "FBATransportationFee")]
    FBATransportationFee,
    #[serde(rename = "FBAFulfillmentCODFee")]
    FBAFulfillmentCODFee,
}

impl Default for Name {
    fn default() -> Name {
        Self::FBAPerUnitFulfillmentFee
    }
}

