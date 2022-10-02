/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFeaturesResult : The payload for the getFeatures operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFeaturesResult {
    /// An array of features.
    #[serde(rename = "features")]
    pub features: Vec<crate::models::Feature>,
}

impl GetFeaturesResult {
    /// The payload for the getFeatures operation.
    pub fn new(features: Vec<crate::models::Feature>) -> GetFeaturesResult {
        GetFeaturesResult {
            features,
        }
    }
}


