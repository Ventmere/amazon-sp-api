/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssociatedItem : Information about an item associated with the service job.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssociatedItem {
    /// The Amazon Standard Identification Number (ASIN) of the item.
    #[serde(default, rename = "asin", skip_serializing_if = "Option::is_none")]
    pub asin: Option<String>,
    /// The title of the item.
    #[serde(default, rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The total number of items included in the order.
    #[serde(default, rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    /// The Amazon-defined identifier for an order placed by the buyer, in 3-7-7 format.
    #[serde(default, rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// The status of the item.
    #[serde(default, rename = "itemStatus", skip_serializing_if = "Option::is_none")]
    pub item_status: Option<ItemStatus>,
    /// The brand name of the item.
    #[serde(default, rename = "brandName", skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
    #[serde(default, rename = "itemDelivery", skip_serializing_if = "Option::is_none")]
    pub item_delivery: Option<Box<crate::models::ItemDelivery>>,
}

impl AssociatedItem {
    /// Information about an item associated with the service job.
    pub fn new() -> AssociatedItem {
        AssociatedItem {
            asin: None,
            title: None,
            quantity: None,
            order_id: None,
            item_status: None,
            brand_name: None,
            item_delivery: None,
        }
    }
}

/// The status of the item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ItemStatus {
    #[serde(rename = "ACTIVE")]
    ACTIVE,
    #[serde(rename = "CANCELLED")]
    CANCELLED,
    #[serde(rename = "SHIPPED")]
    SHIPPED,
    #[serde(rename = "DELIVERED")]
    DELIVERED,
}

impl Default for ItemStatus {
    fn default() -> ItemStatus {
        Self::ACTIVE
    }
}

