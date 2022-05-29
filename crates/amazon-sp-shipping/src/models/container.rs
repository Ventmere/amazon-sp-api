/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Container : Container in the shipment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Container {
    /// The type of physical container being used. (always 'PACKAGE')
    #[serde(rename = "containerType", skip_serializing_if = "Option::is_none")]
    pub container_type: Option<ContainerType>,
    /// An identifier for the container. This must be unique within all the containers in the same shipment.
    #[serde(rename = "containerReferenceId")]
    pub container_reference_id: String,
    #[serde(rename = "value")]
    pub value: Box<crate::models::Currency>,
    #[serde(rename = "dimensions")]
    pub dimensions: Box<crate::models::Dimensions>,
    /// A list of the items in the container.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::ContainerItem>,
    #[serde(rename = "weight")]
    pub weight: Box<crate::models::Weight>,
}

impl Container {
    /// Container in the shipment.
    pub fn new(container_reference_id: String, value: crate::models::Currency, dimensions: crate::models::Dimensions, items: Vec<crate::models::ContainerItem>, weight: crate::models::Weight) -> Container {
        Container {
            container_type: None,
            container_reference_id,
            value: Box::new(value),
            dimensions: Box::new(dimensions),
            items,
            weight: Box::new(weight),
        }
    }
}

/// The type of physical container being used. (always 'PACKAGE')
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContainerType {
    #[serde(rename = "PACKAGE")]
    PACKAGE,
}

impl Default for ContainerType {
    fn default() -> ContainerType {
        Self::PACKAGE
    }
}

