/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderRegulatedInfo : The order's regulated information along with its verification status.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderRegulatedInfo {
    /// An Amazon-defined order identifier, in 3-7-7 format.
    #[serde(rename = "AmazonOrderId")]
    pub amazon_order_id: String,
    #[serde(rename = "RegulatedInformation")]
    pub regulated_information: Box<crate::models::RegulatedInformation>,
    /// When true, the order requires attaching a dosage information label when shipped.
    #[serde(rename = "RequiresDosageLabel")]
    pub requires_dosage_label: bool,
    #[serde(rename = "RegulatedOrderVerificationStatus")]
    pub regulated_order_verification_status: Box<crate::models::RegulatedOrderVerificationStatus>,
}

impl OrderRegulatedInfo {
    /// The order's regulated information along with its verification status.
    pub fn new(amazon_order_id: String, regulated_information: crate::models::RegulatedInformation, requires_dosage_label: bool, regulated_order_verification_status: crate::models::RegulatedOrderVerificationStatus) -> OrderRegulatedInfo {
        OrderRegulatedInfo {
            amazon_order_id,
            regulated_information: Box::new(regulated_information),
            requires_dosage_label,
            regulated_order_verification_status: Box::new(regulated_order_verification_status),
        }
    }
}

