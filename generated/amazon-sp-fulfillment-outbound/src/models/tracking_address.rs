/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TrackingAddress : Address information for tracking the package.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackingAddress {
    /// The city.
    #[serde(default, rename = "city")]
    pub city: String,
    /// The state.
    #[serde(default, rename = "state")]
    pub state: String,
    /// The country.
    #[serde(default, rename = "country")]
    pub country: String,
}

impl TrackingAddress {
    /// Address information for tracking the package.
    pub fn new(city: String, state: String, country: String) -> TrackingAddress {
        TrackingAddress {
            city,
            state,
            country,
        }
    }
}


