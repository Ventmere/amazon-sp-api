/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContainerItem : Item in the container.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContainerItem {
    /// The quantity of the items of this type in the container.
    #[serde(rename = "quantity")]
    pub quantity: f32,
    #[serde(rename = "unitPrice")]
    pub unit_price: Box<crate::models::Currency>,
    #[serde(rename = "unitWeight")]
    pub unit_weight: Box<crate::models::Weight>,
    /// A descriptive title of the item.
    #[serde(rename = "title")]
    pub title: String,
}

impl ContainerItem {
    /// Item in the container.
    pub fn new(quantity: f32, unit_price: crate::models::Currency, unit_weight: crate::models::Weight, title: String) -> ContainerItem {
        ContainerItem {
            quantity,
            unit_price: Box::new(unit_price),
            unit_weight: Box::new(unit_weight),
            title,
        }
    }
}


