/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateServiceDocumentUploadDestination : The response schema for the `createServiceDocumentUploadDestination` operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateServiceDocumentUploadDestination {
    #[serde(default, rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<crate::models::ServiceDocumentUploadDestination>>,
    /// A list of error responses returned when a request is unsuccessful.
    #[serde(default, rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Error>>,
}

impl CreateServiceDocumentUploadDestination {
    /// The response schema for the `createServiceDocumentUploadDestination` operation.
    pub fn new() -> CreateServiceDocumentUploadDestination {
        CreateServiceDocumentUploadDestination {
            payload: None,
            errors: None,
        }
    }
}


