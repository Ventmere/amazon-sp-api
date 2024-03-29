/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TemporarilyUnavailableCarrier : A carrier who is temporarily unavailable, most likely due to a service outage experienced by the carrier.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TemporarilyUnavailableCarrier {
    /// The name of the carrier.
    #[serde(default, rename = "CarrierName")]
    pub carrier_name: String,
}

impl TemporarilyUnavailableCarrier {
    /// A carrier who is temporarily unavailable, most likely due to a service outage experienced by the carrier.
    pub fn new(carrier_name: String) -> TemporarilyUnavailableCarrier {
        TemporarilyUnavailableCarrier {
            carrier_name,
        }
    }
}


