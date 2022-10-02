/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LabelPrepType : The type of label preparation that is required for the inbound shipment.

/// The type of label preparation that is required for the inbound shipment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LabelPrepType {
    #[serde(rename = "NO_LABEL")]
    NOLABEL,
    #[serde(rename = "SELLER_LABEL")]
    SELLERLABEL,
    #[serde(rename = "AMAZON_LABEL")]
    AMAZONLABEL,

}

impl ToString for LabelPrepType {
    fn to_string(&self) -> String {
        match self {
            Self::NOLABEL => String::from("NO_LABEL"),
            Self::SELLERLABEL => String::from("SELLER_LABEL"),
            Self::AMAZONLABEL => String::from("AMAZON_LABEL"),
        }
    }
}

impl Default for LabelPrepType {
    fn default() -> LabelPrepType {
        Self::NOLABEL
    }
}




