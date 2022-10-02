/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TermsAndConditionsNotAcceptedCarrier : A carrier whose terms and conditions have not been accepted by the seller.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TermsAndConditionsNotAcceptedCarrier {
    /// The name of the carrier.
    #[serde(rename = "CarrierName")]
    pub carrier_name: String,
}

impl TermsAndConditionsNotAcceptedCarrier {
    /// A carrier whose terms and conditions have not been accepted by the seller.
    pub fn new(carrier_name: String) -> TermsAndConditionsNotAcceptedCarrier {
        TermsAndConditionsNotAcceptedCarrier {
            carrier_name,
        }
    }
}

