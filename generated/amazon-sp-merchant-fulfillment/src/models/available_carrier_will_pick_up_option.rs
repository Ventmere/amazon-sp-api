/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AvailableCarrierWillPickUpOption : Indicates whether the carrier will pick up the package, and what fee is charged, if any.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AvailableCarrierWillPickUpOption {
    #[serde(rename = "CarrierWillPickUpOption")]
    pub carrier_will_pick_up_option: crate::models::CarrierWillPickUpOption,
    #[serde(rename = "Charge")]
    pub charge: Box<crate::models::CurrencyAmount>,
}

impl AvailableCarrierWillPickUpOption {
    /// Indicates whether the carrier will pick up the package, and what fee is charged, if any.
    pub fn new(carrier_will_pick_up_option: crate::models::CarrierWillPickUpOption, charge: crate::models::CurrencyAmount) -> AvailableCarrierWillPickUpOption {
        AvailableCarrierWillPickUpOption {
            carrier_will_pick_up_option,
            charge: Box::new(charge),
        }
    }
}


