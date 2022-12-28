/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SetAppointmentFulfillmentDataRequest : Input for set appointment fulfillment data operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAppointmentFulfillmentDataRequest {
    #[serde(rename = "fulfillmentTime", skip_serializing_if = "Option::is_none")]
    pub fulfillment_time: Option<Box<crate::models::FulfillmentTime>>,
    /// List of resources that performs or performed job appointment fulfillment.
    #[serde(rename = "appointmentResources", skip_serializing_if = "Option::is_none")]
    pub appointment_resources: Option<Vec<crate::models::AppointmentResource>>,
    /// List of documents captured during service appointment fulfillment.
    #[serde(rename = "fulfillmentDocuments", skip_serializing_if = "Option::is_none")]
    pub fulfillment_documents: Option<Vec<crate::models::FulfillmentDocument>>,
}

impl SetAppointmentFulfillmentDataRequest {
    /// Input for set appointment fulfillment data operation.
    pub fn new() -> SetAppointmentFulfillmentDataRequest {
        SetAppointmentFulfillmentDataRequest {
            fulfillment_time: None,
            appointment_resources: None,
            fulfillment_documents: None,
        }
    }
}

