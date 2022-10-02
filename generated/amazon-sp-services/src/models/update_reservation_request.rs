/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateReservationRequest : Request schema for the `updateReservation` operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateReservationRequest {
    /// Resource (store) identifier.
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "reservation")]
    pub reservation: Box<crate::models::Reservation>,
}

impl UpdateReservationRequest {
    /// Request schema for the `updateReservation` operation.
    pub fn new(resource_id: String, reservation: crate::models::Reservation) -> UpdateReservationRequest {
        UpdateReservationRequest {
            resource_id,
            reservation: Box::new(reservation),
        }
    }
}


