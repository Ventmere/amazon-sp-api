/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AvailableShippingServiceOptions : The available shipping service options.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AvailableShippingServiceOptions {
    /// List of available carrier pickup options.
    #[serde(default, rename = "AvailableCarrierWillPickUpOptions")]
    pub available_carrier_will_pick_up_options: Vec<crate::models::AvailableCarrierWillPickUpOption>,
    /// List of available delivery experience options.
    #[serde(default, rename = "AvailableDeliveryExperienceOptions")]
    pub available_delivery_experience_options: Vec<crate::models::AvailableDeliveryExperienceOption>,
}

impl AvailableShippingServiceOptions {
    /// The available shipping service options.
    pub fn new(available_carrier_will_pick_up_options: Vec<crate::models::AvailableCarrierWillPickUpOption>, available_delivery_experience_options: Vec<crate::models::AvailableDeliveryExperienceOption>) -> AvailableShippingServiceOptions {
        AvailableShippingServiceOptions {
            available_carrier_will_pick_up_options,
            available_delivery_experience_options,
        }
    }
}


