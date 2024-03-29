/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransportStatus : Indicates the status of the Amazon-partnered carrier shipment.

/// Indicates the status of the Amazon-partnered carrier shipment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransportStatus {
    #[serde(rename = "WORKING")]
    WORKING,
    #[serde(rename = "ESTIMATING")]
    ESTIMATING,
    #[serde(rename = "ESTIMATED")]
    ESTIMATED,
    #[serde(rename = "ERROR_ON_ESTIMATING")]
    ERRORONESTIMATING,
    #[serde(rename = "CONFIRMING")]
    CONFIRMING,
    #[serde(rename = "CONFIRMED")]
    CONFIRMED,
    #[serde(rename = "ERROR_ON_CONFIRMING")]
    ERRORONCONFIRMING,
    #[serde(rename = "VOIDING")]
    VOIDING,
    #[serde(rename = "VOIDED")]
    VOIDED,
    #[serde(rename = "ERROR_IN_VOIDING")]
    ERRORINVOIDING,
    #[serde(rename = "ERROR")]
    ERROR,

}

impl ToString for TransportStatus {
    fn to_string(&self) -> String {
        match self {
            Self::WORKING => String::from("WORKING"),
            Self::ESTIMATING => String::from("ESTIMATING"),
            Self::ESTIMATED => String::from("ESTIMATED"),
            Self::ERRORONESTIMATING => String::from("ERROR_ON_ESTIMATING"),
            Self::CONFIRMING => String::from("CONFIRMING"),
            Self::CONFIRMED => String::from("CONFIRMED"),
            Self::ERRORONCONFIRMING => String::from("ERROR_ON_CONFIRMING"),
            Self::VOIDING => String::from("VOIDING"),
            Self::VOIDED => String::from("VOIDED"),
            Self::ERRORINVOIDING => String::from("ERROR_IN_VOIDING"),
            Self::ERROR => String::from("ERROR"),
        }
    }
}

impl Default for TransportStatus {
    fn default() -> TransportStatus {
        Self::WORKING
    }
}




