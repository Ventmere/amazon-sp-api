/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartneredEstimate : The estimated shipping cost for a shipment using an Amazon-partnered carrier.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartneredEstimate {
    #[serde(default, rename = "Amount")]
    pub amount: Box<crate::models::Amount>,
    #[serde(default, rename = "ConfirmDeadline", skip_serializing_if = "Option::is_none")]
    pub confirm_deadline: Option<String>,
    #[serde(default, rename = "VoidDeadline", skip_serializing_if = "Option::is_none")]
    pub void_deadline: Option<String>,
}

impl PartneredEstimate {
    /// The estimated shipping cost for a shipment using an Amazon-partnered carrier.
    pub fn new(amount: crate::models::Amount) -> PartneredEstimate {
        PartneredEstimate {
            amount: Box::new(amount),
            confirm_deadline: None,
            void_deadline: None,
        }
    }
}


