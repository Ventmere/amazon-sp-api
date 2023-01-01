/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServiceJob : The job details of a service.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceJob {
    /// The date and time of the creation of the job in ISO 8601 format.
    #[serde(default, rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// Amazon identifier for the service job.
    #[serde(default, rename = "serviceJobId", skip_serializing_if = "Option::is_none")]
    pub service_job_id: Option<String>,
    /// The status of the service job.
    #[serde(default, rename = "serviceJobStatus", skip_serializing_if = "Option::is_none")]
    pub service_job_status: Option<ServiceJobStatus>,
    #[serde(default, rename = "scopeOfWork", skip_serializing_if = "Option::is_none")]
    pub scope_of_work: Option<Box<crate::models::ScopeOfWork>>,
    #[serde(default, rename = "seller", skip_serializing_if = "Option::is_none")]
    pub seller: Option<Box<crate::models::Seller>>,
    #[serde(default, rename = "serviceJobProvider", skip_serializing_if = "Option::is_none")]
    pub service_job_provider: Option<Box<crate::models::ServiceJobProvider>>,
    /// A list of appointment windows preferred by the buyer. Included only if the buyer selected appointment windows when creating the order.
    #[serde(default, rename = "preferredAppointmentTimes", skip_serializing_if = "Option::is_none")]
    pub preferred_appointment_times: Option<Vec<crate::models::AppointmentTime>>,
    /// A list of appointments.
    #[serde(default, rename = "appointments", skip_serializing_if = "Option::is_none")]
    pub appointments: Option<Vec<crate::models::Appointment>>,
    /// The Amazon-defined identifier for an order placed by the buyer, in 3-7-7 format.
    #[serde(default, rename = "serviceOrderId", skip_serializing_if = "Option::is_none")]
    pub service_order_id: Option<String>,
    /// The marketplace identifier.
    #[serde(default, rename = "marketplaceId", skip_serializing_if = "Option::is_none")]
    pub marketplace_id: Option<String>,
    /// The Amazon-defined identifier for the region scope.
    #[serde(default, rename = "storeId", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    #[serde(default, rename = "buyer", skip_serializing_if = "Option::is_none")]
    pub buyer: Option<Box<crate::models::Buyer>>,
    /// A list of items associated with the service job.
    #[serde(default, rename = "associatedItems", skip_serializing_if = "Option::is_none")]
    pub associated_items: Option<Vec<crate::models::AssociatedItem>>,
    #[serde(default, rename = "serviceLocation", skip_serializing_if = "Option::is_none")]
    pub service_location: Option<Box<crate::models::ServiceLocation>>,
}

impl ServiceJob {
    /// The job details of a service.
    pub fn new() -> ServiceJob {
        ServiceJob {
            create_time: None,
            service_job_id: None,
            service_job_status: None,
            scope_of_work: None,
            seller: None,
            service_job_provider: None,
            preferred_appointment_times: None,
            appointments: None,
            service_order_id: None,
            marketplace_id: None,
            store_id: None,
            buyer: None,
            associated_items: None,
            service_location: None,
        }
    }
}

/// The status of the service job.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceJobStatus {
    #[serde(rename = "NOT_SERVICED")]
    NOTSERVICED,
    #[serde(rename = "CANCELLED")]
    CANCELLED,
    #[serde(rename = "COMPLETED")]
    COMPLETED,
    #[serde(rename = "PENDING_SCHEDULE")]
    PENDINGSCHEDULE,
    #[serde(rename = "NOT_FULFILLABLE")]
    NOTFULFILLABLE,
    #[serde(rename = "HOLD")]
    HOLD,
    #[serde(rename = "PAYMENT_DECLINED")]
    PAYMENTDECLINED,
}

impl Default for ServiceJobStatus {
    fn default() -> ServiceJobStatus {
        Self::NOTSERVICED
    }
}

