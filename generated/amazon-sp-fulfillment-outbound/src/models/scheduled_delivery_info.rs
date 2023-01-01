/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScheduledDeliveryInfo : Delivery information for a scheduled delivery.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScheduledDeliveryInfo {
    /// The time zone of the destination address for the fulfillment order preview. Must be an IANA time zone name. Example: Asia/Tokyo.
    #[serde(default, rename = "deliveryTimeZone")]
    pub delivery_time_zone: String,
    /// An array of delivery windows.
    #[serde(default, rename = "deliveryWindows")]
    pub delivery_windows: Vec<crate::models::DeliveryWindow>,
}

impl ScheduledDeliveryInfo {
    /// Delivery information for a scheduled delivery.
    pub fn new(delivery_time_zone: String, delivery_windows: Vec<crate::models::DeliveryWindow>) -> ScheduledDeliveryInfo {
        ScheduledDeliveryInfo {
            delivery_time_zone,
            delivery_windows,
        }
    }
}


