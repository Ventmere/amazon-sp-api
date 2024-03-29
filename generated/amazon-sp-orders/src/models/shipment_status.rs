/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ShipmentStatus : The shipment status to apply.

/// The shipment status to apply.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShipmentStatus {
    #[serde(rename = "ReadyForPickup")]
    ReadyForPickup,
    #[serde(rename = "PickedUp")]
    PickedUp,
    #[serde(rename = "RefusedPickup")]
    RefusedPickup,

}

impl ToString for ShipmentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::ReadyForPickup => String::from("ReadyForPickup"),
            Self::PickedUp => String::from("PickedUp"),
            Self::RefusedPickup => String::from("RefusedPickup"),
        }
    }
}

impl Default for ShipmentStatus {
    fn default() -> ShipmentStatus {
        Self::ReadyForPickup
    }
}




