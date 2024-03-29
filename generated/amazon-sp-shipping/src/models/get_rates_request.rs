/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetRatesRequest : The payload schema for the getRates operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetRatesRequest {
    #[serde(default, rename = "shipTo")]
    pub ship_to: Box<crate::models::Address>,
    #[serde(default, rename = "shipFrom")]
    pub ship_from: Box<crate::models::Address>,
    /// A list of service types that can be used to send the shipment.
    #[serde(default, rename = "serviceTypes")]
    pub service_types: Vec<crate::models::ServiceType>,
    /// The start date and time. This defaults to the current date and time.
    #[serde(default, rename = "shipDate", skip_serializing_if = "Option::is_none")]
    pub ship_date: Option<String>,
    /// A list of container specifications.
    #[serde(default, rename = "containerSpecifications")]
    pub container_specifications: Vec<crate::models::ContainerSpecification>,
}

impl GetRatesRequest {
    /// The payload schema for the getRates operation.
    pub fn new(ship_to: crate::models::Address, ship_from: crate::models::Address, service_types: Vec<crate::models::ServiceType>, container_specifications: Vec<crate::models::ContainerSpecification>) -> GetRatesRequest {
        GetRatesRequest {
            ship_to: Box::new(ship_to),
            ship_from: Box::new(ship_from),
            service_types,
            ship_date: None,
            container_specifications,
        }
    }
}


