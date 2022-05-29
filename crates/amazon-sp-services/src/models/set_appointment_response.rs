/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SetAppointmentResponse : Response schema for add or reschedule appointment operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAppointmentResponse {
    /// The appointment identifier.
    #[serde(rename = "appointmentId", skip_serializing_if = "Option::is_none")]
    pub appointment_id: Option<String>,
    /// A list of warnings returned in the sucessful execution response of an API request.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<crate::models::Warning>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl SetAppointmentResponse {
    /// Response schema for add or reschedule appointment operation.
    pub fn new() -> SetAppointmentResponse {
        SetAppointmentResponse {
            appointment_id: None,
            warnings: None,
            errors: None,
        }
    }
}


