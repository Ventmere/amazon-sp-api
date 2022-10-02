/*
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentDates : Shipment dates.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipmentDates {
    /// Time by which the vendor is required to ship the order.
    #[serde(rename = "requiredShipDate")]
    pub required_ship_date: String,
    /// Delivery date promised to the Amazon customer.
    #[serde(rename = "promisedDeliveryDate", skip_serializing_if = "Option::is_none")]
    pub promised_delivery_date: Option<String>,
}

impl ShipmentDates {
    /// Shipment dates.
    pub fn new(required_ship_date: String) -> ShipmentDates {
        ShipmentDates {
            required_ship_date,
            promised_delivery_date: None,
        }
    }
}


