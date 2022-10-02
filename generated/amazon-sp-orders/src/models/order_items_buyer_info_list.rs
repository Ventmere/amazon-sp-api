/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderItemsBuyerInfoList : A single order item's buyer information list with the order ID.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderItemsBuyerInfoList {
    /// A single order item's buyer information list.
    #[serde(rename = "OrderItems")]
    pub order_items: Vec<crate::models::OrderItemBuyerInfo>,
    /// When present and not empty, pass this string token in the next request to return the next response page.
    #[serde(rename = "NextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// An Amazon-defined order identifier, in 3-7-7 format.
    #[serde(rename = "AmazonOrderId")]
    pub amazon_order_id: String,
}

impl OrderItemsBuyerInfoList {
    /// A single order item's buyer information list with the order ID.
    pub fn new(order_items: Vec<crate::models::OrderItemBuyerInfo>, amazon_order_id: String) -> OrderItemsBuyerInfoList {
        OrderItemsBuyerInfoList {
            order_items,
            next_token: None,
            amazon_order_id,
        }
    }
}

