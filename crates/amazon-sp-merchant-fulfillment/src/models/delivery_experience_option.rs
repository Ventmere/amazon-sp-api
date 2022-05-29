/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeliveryExperienceOption : The delivery confirmation level.

/// The delivery confirmation level.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeliveryExperienceOption {
    #[serde(rename = "DeliveryConfirmationWithAdultSignature")]
    DeliveryConfirmationWithAdultSignature,
    #[serde(rename = "DeliveryConfirmationWithSignature")]
    DeliveryConfirmationWithSignature,
    #[serde(rename = "DeliveryConfirmationWithoutSignature")]
    DeliveryConfirmationWithoutSignature,
    #[serde(rename = "NoTracking")]
    NoTracking,
    #[serde(rename = "NoPreference")]
    NoPreference,

}

impl ToString for DeliveryExperienceOption {
    fn to_string(&self) -> String {
        match self {
            Self::DeliveryConfirmationWithAdultSignature => String::from("DeliveryConfirmationWithAdultSignature"),
            Self::DeliveryConfirmationWithSignature => String::from("DeliveryConfirmationWithSignature"),
            Self::DeliveryConfirmationWithoutSignature => String::from("DeliveryConfirmationWithoutSignature"),
            Self::NoTracking => String::from("NoTracking"),
            Self::NoPreference => String::from("NoPreference"),
        }
    }
}

impl Default for DeliveryExperienceOption {
    fn default() -> DeliveryExperienceOption {
        Self::DeliveryConfirmationWithAdultSignature
    }
}




