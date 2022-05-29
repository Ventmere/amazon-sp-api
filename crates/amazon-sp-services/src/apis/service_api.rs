/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`add_appointment_for_service_job_by_service_job_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddAppointmentForServiceJobByServiceJobIdError {
    Status400(crate::models::SetAppointmentResponse),
    Status403(crate::models::SetAppointmentResponse),
    Status404(crate::models::SetAppointmentResponse),
    Status413(crate::models::SetAppointmentResponse),
    Status415(crate::models::SetAppointmentResponse),
    Status422(crate::models::SetAppointmentResponse),
    Status429(crate::models::SetAppointmentResponse),
    Status500(crate::models::SetAppointmentResponse),
    Status503(crate::models::SetAppointmentResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cancel_service_job_by_service_job_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelServiceJobByServiceJobIdError {
    Status400(crate::models::CancelServiceJobByServiceJobIdResponse),
    Status403(crate::models::CancelServiceJobByServiceJobIdResponse),
    Status404(crate::models::CancelServiceJobByServiceJobIdResponse),
    Status413(crate::models::CancelServiceJobByServiceJobIdResponse),
    Status415(crate::models::CancelServiceJobByServiceJobIdResponse),
    Status422(crate::models::CancelServiceJobByServiceJobIdResponse),
    Status429(crate::models::CancelServiceJobByServiceJobIdResponse),
    Status500(crate::models::CancelServiceJobByServiceJobIdResponse),
    Status503(crate::models::CancelServiceJobByServiceJobIdResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`complete_service_job_by_service_job_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompleteServiceJobByServiceJobIdError {
    Status400(crate::models::CompleteServiceJobByServiceJobIdResponse),
    Status403(crate::models::CompleteServiceJobByServiceJobIdResponse),
    Status404(crate::models::CompleteServiceJobByServiceJobIdResponse),
    Status413(crate::models::CompleteServiceJobByServiceJobIdResponse),
    Status415(crate::models::CompleteServiceJobByServiceJobIdResponse),
    Status422(crate::models::CompleteServiceJobByServiceJobIdResponse),
    Status429(crate::models::CompleteServiceJobByServiceJobIdResponse),
    Status500(crate::models::CompleteServiceJobByServiceJobIdResponse),
    Status503(crate::models::CompleteServiceJobByServiceJobIdResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_service_job_by_service_job_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceJobByServiceJobIdError {
    Status400(crate::models::GetServiceJobByServiceJobIdResponse),
    Status403(crate::models::GetServiceJobByServiceJobIdResponse),
    Status404(crate::models::GetServiceJobByServiceJobIdResponse),
    Status413(crate::models::GetServiceJobByServiceJobIdResponse),
    Status415(crate::models::GetServiceJobByServiceJobIdResponse),
    Status422(crate::models::GetServiceJobByServiceJobIdResponse),
    Status429(crate::models::GetServiceJobByServiceJobIdResponse),
    Status500(crate::models::GetServiceJobByServiceJobIdResponse),
    Status503(crate::models::GetServiceJobByServiceJobIdResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_service_jobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceJobsError {
    Status400(crate::models::GetServiceJobsResponse),
    Status403(crate::models::GetServiceJobsResponse),
    Status404(crate::models::GetServiceJobsResponse),
    Status413(crate::models::GetServiceJobsResponse),
    Status415(crate::models::GetServiceJobsResponse),
    Status429(crate::models::GetServiceJobsResponse),
    Status500(crate::models::GetServiceJobsResponse),
    Status503(crate::models::GetServiceJobsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reschedule_appointment_for_service_job_by_service_job_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RescheduleAppointmentForServiceJobByServiceJobIdError {
    Status400(crate::models::SetAppointmentResponse),
    Status403(crate::models::SetAppointmentResponse),
    Status404(crate::models::SetAppointmentResponse),
    Status413(crate::models::SetAppointmentResponse),
    Status415(crate::models::SetAppointmentResponse),
    Status422(crate::models::SetAppointmentResponse),
    Status429(crate::models::SetAppointmentResponse),
    Status500(crate::models::SetAppointmentResponse),
    Status503(crate::models::SetAppointmentResponse),
    UnknownValue(serde_json::Value),
}


/// Adds an appointment to the service job indicated by the service job identifier you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn add_appointment_for_service_job_by_service_job_id(configuration: &configuration::Configuration, service_job_id: &str, body: crate::models::AddAppointmentRequest) -> Result<crate::models::SetAppointmentResponse, Error<AddAppointmentForServiceJobByServiceJobIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/v1/serviceJobs/{serviceJobId}/appointments", local_var_configuration.base_path, serviceJobId=crate::apis::urlencode(service_job_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddAppointmentForServiceJobByServiceJobIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Cancels the service job indicated by the service job identifier you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn cancel_service_job_by_service_job_id(configuration: &configuration::Configuration, service_job_id: &str, cancellation_reason_code: &str) -> Result<crate::models::CancelServiceJobByServiceJobIdResponse, Error<CancelServiceJobByServiceJobIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/v1/serviceJobs/{serviceJobId}/cancellations", local_var_configuration.base_path, serviceJobId=crate::apis::urlencode(service_job_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("cancellationReasonCode", &cancellation_reason_code.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CancelServiceJobByServiceJobIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Completes the service job indicated by the service job identifier you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn complete_service_job_by_service_job_id(configuration: &configuration::Configuration, service_job_id: &str) -> Result<crate::models::CompleteServiceJobByServiceJobIdResponse, Error<CompleteServiceJobByServiceJobIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/v1/serviceJobs/{serviceJobId}/completions", local_var_configuration.base_path, serviceJobId=crate::apis::urlencode(service_job_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CompleteServiceJobByServiceJobIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets service job details for the service job indicated by the service job identifier you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 20 | 40 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_service_job_by_service_job_id(configuration: &configuration::Configuration, service_job_id: &str) -> Result<crate::models::GetServiceJobByServiceJobIdResponse, Error<GetServiceJobByServiceJobIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/v1/serviceJobs/{serviceJobId}", local_var_configuration.base_path, serviceJobId=crate::apis::urlencode(service_job_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetServiceJobByServiceJobIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets service job details for the specified filter query.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 40 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_service_jobs(configuration: &configuration::Configuration, marketplace_ids: Vec<String>, service_order_ids: Option<Vec<String>>, service_job_status: Option<Vec<String>>, page_token: Option<&str>, page_size: Option<i32>, sort_field: Option<&str>, sort_order: Option<&str>, created_after: Option<&str>, created_before: Option<&str>, last_updated_after: Option<&str>, last_updated_before: Option<&str>, schedule_start_date: Option<&str>, schedule_end_date: Option<&str>) -> Result<crate::models::GetServiceJobsResponse, Error<GetServiceJobsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/v1/serviceJobs", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = service_order_ids {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("serviceOrderIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("serviceOrderIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = service_job_status {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("serviceJobStatus".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("serviceJobStatus", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_field {
        local_var_req_builder = local_var_req_builder.query(&[("sortField", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_order {
        local_var_req_builder = local_var_req_builder.query(&[("sortOrder", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_after {
        local_var_req_builder = local_var_req_builder.query(&[("createdAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_before {
        local_var_req_builder = local_var_req_builder.query(&[("createdBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = last_updated_after {
        local_var_req_builder = local_var_req_builder.query(&[("lastUpdatedAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = last_updated_before {
        local_var_req_builder = local_var_req_builder.query(&[("lastUpdatedBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = schedule_start_date {
        local_var_req_builder = local_var_req_builder.query(&[("scheduleStartDate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = schedule_end_date {
        local_var_req_builder = local_var_req_builder.query(&[("scheduleEndDate", &local_var_str.to_string())]);
    }
    local_var_req_builder = match "csv" {
        "multi" => local_var_req_builder.query(&marketplace_ids.into_iter().map(|p| ("marketplaceIds".to_owned(), p)).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => local_var_req_builder.query(&[("marketplaceIds", &marketplace_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetServiceJobsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Reschedules an appointment for the service job indicated by the service job identifier you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn reschedule_appointment_for_service_job_by_service_job_id(configuration: &configuration::Configuration, service_job_id: &str, appointment_id: &str, body: crate::models::RescheduleAppointmentRequest) -> Result<crate::models::SetAppointmentResponse, Error<RescheduleAppointmentForServiceJobByServiceJobIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/v1/serviceJobs/{serviceJobId}/appointments/{appointmentId}", local_var_configuration.base_path, serviceJobId=crate::apis::urlencode(service_job_id), appointmentId=crate::apis::urlencode(appointment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RescheduleAppointmentForServiceJobByServiceJobIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

