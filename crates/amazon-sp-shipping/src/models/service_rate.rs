/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServiceRate : The specific rate for a shipping service, or null if no service available.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceRate {
    #[serde(rename = "totalCharge")]
    pub total_charge: Box<crate::models::Currency>,
    #[serde(rename = "billableWeight")]
    pub billable_weight: Box<crate::models::Weight>,
    #[serde(rename = "serviceType")]
    pub service_type: crate::models::ServiceType,
    #[serde(rename = "promise")]
    pub promise: Box<crate::models::ShippingPromiseSet>,
}

impl ServiceRate {
    /// The specific rate for a shipping service, or null if no service available.
    pub fn new(total_charge: crate::models::Currency, billable_weight: crate::models::Weight, service_type: crate::models::ServiceType, promise: crate::models::ShippingPromiseSet) -> ServiceRate {
        ServiceRate {
            total_charge: Box::new(total_charge),
            billable_weight: Box::new(billable_weight),
            service_type,
            promise: Box::new(promise),
        }
    }
}


