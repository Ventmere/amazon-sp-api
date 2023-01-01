/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AppointmentResource : The resource that performs or performed appointment fulfillment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AppointmentResource {
    /// The resource identifier.
    #[serde(default, rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl AppointmentResource {
    /// The resource that performs or performed appointment fulfillment.
    pub fn new() -> AppointmentResource {
        AppointmentResource {
            resource_id: None,
        }
    }
}


