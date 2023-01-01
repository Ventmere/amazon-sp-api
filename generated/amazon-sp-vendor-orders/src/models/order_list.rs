/*
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderList {
    #[serde(default, rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(default, rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<crate::models::Order>>,
}

impl OrderList {
    pub fn new() -> OrderList {
        OrderList {
            pagination: None,
            orders: None,
        }
    }
}


