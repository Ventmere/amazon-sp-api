/*
 * Selling Partner API for Sales
 *
 * The Selling Partner API for Sales provides APIs related to sales performance.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderMetricsInterval : Contains order metrics.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderMetricsInterval {
    /// The interval of time based on requested granularity (ex. Hour, Day, etc.) If this is the first or the last interval from the list, it might contain incomplete data if the requested interval doesn't align with the requested granularity (ex. request interval 2018-09-01T02:00:00Z--2018-09-04T19:00:00Z and granularity day will result in Sept 1st UTC day and Sept 4th UTC days having partial data).
    #[serde(default, rename = "interval")]
    pub interval: String,
    /// The number of units in orders based on the specified filters.
    #[serde(default, rename = "unitCount")]
    pub unit_count: i32,
    /// The number of order items based on the specified filters.
    #[serde(default, rename = "orderItemCount")]
    pub order_item_count: i32,
    /// The number of orders based on the specified filters.
    #[serde(default, rename = "orderCount")]
    pub order_count: i32,
    #[serde(default, rename = "averageUnitPrice")]
    pub average_unit_price: Box<crate::models::Money>,
    #[serde(default, rename = "totalSales")]
    pub total_sales: Box<crate::models::Money>,
}

impl OrderMetricsInterval {
    /// Contains order metrics.
    pub fn new(interval: String, unit_count: i32, order_item_count: i32, order_count: i32, average_unit_price: crate::models::Money, total_sales: crate::models::Money) -> OrderMetricsInterval {
        OrderMetricsInterval {
            interval,
            unit_count,
            order_item_count,
            order_count,
            average_unit_price: Box::new(average_unit_price),
            total_sales: Box::new(total_sales),
        }
    }
}


