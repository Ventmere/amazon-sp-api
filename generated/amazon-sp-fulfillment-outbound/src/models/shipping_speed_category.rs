/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShippingSpeedCategory : The shipping method used for the fulfillment order.

/// The shipping method used for the fulfillment order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShippingSpeedCategory {
    #[serde(rename = "Standard")]
    Standard,
    #[serde(rename = "Expedited")]
    Expedited,
    #[serde(rename = "Priority")]
    Priority,
    #[serde(rename = "ScheduledDelivery")]
    ScheduledDelivery,

}

impl ToString for ShippingSpeedCategory {
    fn to_string(&self) -> String {
        match self {
            Self::Standard => String::from("Standard"),
            Self::Expedited => String::from("Expedited"),
            Self::Priority => String::from("Priority"),
            Self::ScheduledDelivery => String::from("ScheduledDelivery"),
        }
    }
}

impl Default for ShippingSpeedCategory {
    fn default() -> ShippingSpeedCategory {
        Self::Standard
    }
}



