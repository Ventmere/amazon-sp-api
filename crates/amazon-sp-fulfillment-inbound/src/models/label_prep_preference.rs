/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LabelPrepPreference : The preference for label preparation for an inbound shipment.

/// The preference for label preparation for an inbound shipment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LabelPrepPreference {
    #[serde(rename = "SELLER_LABEL")]
    SELLERLABEL,
    #[serde(rename = "AMAZON_LABEL_ONLY")]
    AMAZONLABELONLY,
    #[serde(rename = "AMAZON_LABEL_PREFERRED")]
    AMAZONLABELPREFERRED,

}

impl ToString for LabelPrepPreference {
    fn to_string(&self) -> String {
        match self {
            Self::SELLERLABEL => String::from("SELLER_LABEL"),
            Self::AMAZONLABELONLY => String::from("AMAZON_LABEL_ONLY"),
            Self::AMAZONLABELPREFERRED => String::from("AMAZON_LABEL_PREFERRED"),
        }
    }
}

impl Default for LabelPrepPreference {
    fn default() -> LabelPrepPreference {
        Self::SELLERLABEL
    }
}




