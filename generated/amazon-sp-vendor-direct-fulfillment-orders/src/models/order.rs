/*
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Order {
    /// The purchase order number for this order. Formatting Notes: alpha-numeric code.
    #[serde(default, rename = "purchaseOrderNumber")]
    pub purchase_order_number: String,
    #[serde(default, rename = "orderDetails", skip_serializing_if = "Option::is_none")]
    pub order_details: Option<Box<crate::models::OrderDetails>>,
}

impl Order {
    pub fn new(purchase_order_number: String) -> Order {
        Order {
            purchase_order_number,
            order_details: None,
        }
    }
}


