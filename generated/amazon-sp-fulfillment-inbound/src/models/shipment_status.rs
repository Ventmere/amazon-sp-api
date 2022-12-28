/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentStatus : Indicates the status of the inbound shipment. When used with the createInboundShipment operation, WORKING is the only valid value. When used with the updateInboundShipment operation, possible values are WORKING, SHIPPED or CANCELLED.

/// Indicates the status of the inbound shipment. When used with the createInboundShipment operation, WORKING is the only valid value. When used with the updateInboundShipment operation, possible values are WORKING, SHIPPED or CANCELLED.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShipmentStatus {
    #[serde(rename = "WORKING")]
    WORKING,
    #[serde(rename = "SHIPPED")]
    SHIPPED,
    #[serde(rename = "RECEIVING")]
    RECEIVING,
    #[serde(rename = "CANCELLED")]
    CANCELLED,
    #[serde(rename = "DELETED")]
    DELETED,
    #[serde(rename = "CLOSED")]
    CLOSED,
    #[serde(rename = "ERROR")]
    ERROR,
    #[serde(rename = "IN_TRANSIT")]
    INTRANSIT,
    #[serde(rename = "DELIVERED")]
    DELIVERED,
    #[serde(rename = "CHECKED_IN")]
    CHECKEDIN,

}

impl ToString for ShipmentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::WORKING => String::from("WORKING"),
            Self::SHIPPED => String::from("SHIPPED"),
            Self::RECEIVING => String::from("RECEIVING"),
            Self::CANCELLED => String::from("CANCELLED"),
            Self::DELETED => String::from("DELETED"),
            Self::CLOSED => String::from("CLOSED"),
            Self::ERROR => String::from("ERROR"),
            Self::INTRANSIT => String::from("IN_TRANSIT"),
            Self::DELIVERED => String::from("DELIVERED"),
            Self::CHECKEDIN => String::from("CHECKED_IN"),
        }
    }
}

impl Default for ShipmentStatus {
    fn default() -> ShipmentStatus {
        Self::WORKING
    }
}



