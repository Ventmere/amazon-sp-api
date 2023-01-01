/*
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderDetails : Details of an order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderDetails {
    /// The customer order number.
    #[serde(default, rename = "customerOrderNumber")]
    pub customer_order_number: String,
    /// The date the order was placed. This field is expected to be in ISO-8601 date/time format, for example:2018-07-16T23:00:00Z/ 2018-07-16T23:00:00-05:00 /2018-07-16T23:00:00-08:00. If no time zone is specified, UTC should be assumed.
    #[serde(default, rename = "orderDate")]
    pub order_date: String,
    /// Current status of the order.
    #[serde(default, rename = "orderStatus", skip_serializing_if = "Option::is_none")]
    pub order_status: Option<OrderStatus>,
    #[serde(default, rename = "shipmentDetails")]
    pub shipment_details: Box<crate::models::ShipmentDetails>,
    #[serde(default, rename = "taxTotal", skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<Box<crate::models::OrderDetailsTaxTotal>>,
    #[serde(default, rename = "sellingParty")]
    pub selling_party: Box<crate::models::PartyIdentification>,
    #[serde(default, rename = "shipFromParty")]
    pub ship_from_party: Box<crate::models::PartyIdentification>,
    #[serde(default, rename = "shipToParty")]
    pub ship_to_party: Box<crate::models::Address>,
    #[serde(default, rename = "billToParty")]
    pub bill_to_party: Box<crate::models::PartyIdentification>,
    /// A list of items in this purchase order.
    #[serde(default, rename = "items")]
    pub items: Vec<crate::models::OrderItem>,
}

impl OrderDetails {
    /// Details of an order.
    pub fn new(customer_order_number: String, order_date: String, shipment_details: crate::models::ShipmentDetails, selling_party: crate::models::PartyIdentification, ship_from_party: crate::models::PartyIdentification, ship_to_party: crate::models::Address, bill_to_party: crate::models::PartyIdentification, items: Vec<crate::models::OrderItem>) -> OrderDetails {
        OrderDetails {
            customer_order_number,
            order_date,
            order_status: None,
            shipment_details: Box::new(shipment_details),
            tax_total: None,
            selling_party: Box::new(selling_party),
            ship_from_party: Box::new(ship_from_party),
            ship_to_party: Box::new(ship_to_party),
            bill_to_party: Box::new(bill_to_party),
            items,
        }
    }
}

/// Current status of the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "NEW")]
    NEW,
    #[serde(rename = "SHIPPED")]
    SHIPPED,
    #[serde(rename = "ACCEPTED")]
    ACCEPTED,
    #[serde(rename = "CANCELLED")]
    CANCELLED,
}

impl Default for OrderStatus {
    fn default() -> OrderStatus {
        Self::NEW
    }
}

