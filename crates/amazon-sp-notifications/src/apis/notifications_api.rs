/*
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.  For more information, see the [Notifications Use Case Guide](doc:notifications-api-v1-use-case-guide).
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_destination`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDestinationError {
    Status400(crate::models::CreateDestinationResponse),
    Status403(crate::models::CreateDestinationResponse),
    Status404(crate::models::CreateDestinationResponse),
    Status409(crate::models::CreateDestinationResponse),
    Status413(crate::models::CreateDestinationResponse),
    Status415(crate::models::CreateDestinationResponse),
    Status429(crate::models::CreateDestinationResponse),
    Status500(crate::models::CreateDestinationResponse),
    Status503(crate::models::CreateDestinationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSubscriptionError {
    Status400(crate::models::CreateSubscriptionResponse),
    Status403(crate::models::CreateSubscriptionResponse),
    Status404(crate::models::CreateSubscriptionResponse),
    Status409(crate::models::CreateSubscriptionResponse),
    Status413(crate::models::CreateSubscriptionResponse),
    Status415(crate::models::CreateSubscriptionResponse),
    Status429(crate::models::CreateSubscriptionResponse),
    Status500(crate::models::CreateSubscriptionResponse),
    Status503(crate::models::CreateSubscriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_destination`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDestinationError {
    Status400(crate::models::DeleteDestinationResponse),
    Status403(crate::models::DeleteDestinationResponse),
    Status404(crate::models::DeleteDestinationResponse),
    Status409(crate::models::DeleteDestinationResponse),
    Status413(crate::models::DeleteDestinationResponse),
    Status415(crate::models::DeleteDestinationResponse),
    Status429(crate::models::DeleteDestinationResponse),
    Status500(crate::models::DeleteDestinationResponse),
    Status503(crate::models::DeleteDestinationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_subscription_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSubscriptionByIdError {
    Status400(crate::models::DeleteSubscriptionByIdResponse),
    Status403(crate::models::DeleteSubscriptionByIdResponse),
    Status404(crate::models::DeleteSubscriptionByIdResponse),
    Status409(crate::models::DeleteSubscriptionByIdResponse),
    Status413(crate::models::DeleteSubscriptionByIdResponse),
    Status415(crate::models::DeleteSubscriptionByIdResponse),
    Status429(crate::models::DeleteSubscriptionByIdResponse),
    Status500(crate::models::DeleteSubscriptionByIdResponse),
    Status503(crate::models::DeleteSubscriptionByIdResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_destination`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDestinationError {
    Status400(crate::models::GetDestinationResponse),
    Status403(crate::models::GetDestinationResponse),
    Status404(crate::models::GetDestinationResponse),
    Status409(crate::models::GetDestinationResponse),
    Status413(crate::models::GetDestinationResponse),
    Status415(crate::models::GetDestinationResponse),
    Status429(crate::models::GetDestinationResponse),
    Status500(crate::models::GetDestinationResponse),
    Status503(crate::models::GetDestinationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_destinations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDestinationsError {
    Status400(crate::models::GetDestinationsResponse),
    Status403(crate::models::GetDestinationsResponse),
    Status404(crate::models::GetDestinationsResponse),
    Status409(crate::models::GetDestinationsResponse),
    Status413(crate::models::GetDestinationsResponse),
    Status415(crate::models::GetDestinationsResponse),
    Status429(crate::models::GetDestinationsResponse),
    Status500(crate::models::GetDestinationsResponse),
    Status503(crate::models::GetDestinationsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSubscriptionError {
    Status400(crate::models::GetSubscriptionResponse),
    Status403(crate::models::GetSubscriptionResponse),
    Status404(crate::models::GetSubscriptionResponse),
    Status413(crate::models::GetSubscriptionResponse),
    Status415(crate::models::GetSubscriptionResponse),
    Status429(crate::models::GetSubscriptionResponse),
    Status500(crate::models::GetSubscriptionResponse),
    Status503(crate::models::GetSubscriptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_subscription_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSubscriptionByIdError {
    Status400(crate::models::GetSubscriptionByIdResponse),
    Status403(crate::models::GetSubscriptionByIdResponse),
    Status404(crate::models::GetSubscriptionResponse),
    Status409(crate::models::GetSubscriptionByIdResponse),
    Status413(crate::models::GetSubscriptionByIdResponse),
    Status415(crate::models::GetSubscriptionByIdResponse),
    Status429(crate::models::GetSubscriptionByIdResponse),
    Status500(crate::models::GetSubscriptionByIdResponse),
    Status503(crate::models::GetSubscriptionByIdResponse),
    UnknownValue(serde_json::Value),
}


/// Creates a destination resource to receive notifications. The createDestination API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn create_destination(configuration: &configuration::Configuration, body: crate::models::CreateDestinationRequest) -> Result<crate::models::CreateDestinationResponse, Error<CreateDestinationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/notifications/v1/destinations", local_var_configuration.base_path);
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
        let local_var_entity: Option<CreateDestinationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a subscription for the specified notification type to be delivered to the specified destination. Before you can subscribe, you must first create the destination by calling the createDestination operation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn create_subscription(configuration: &configuration::Configuration, notification_type: &str, body: crate::models::CreateSubscriptionRequest) -> Result<crate::models::CreateSubscriptionResponse, Error<CreateSubscriptionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/notifications/v1/subscriptions/{notificationType}", local_var_configuration.base_path, notificationType=crate::apis::urlencode(notification_type));
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
        let local_var_entity: Option<CreateSubscriptionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes the destination that you specify. The deleteDestination API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn delete_destination(configuration: &configuration::Configuration, destination_id: &str) -> Result<crate::models::DeleteDestinationResponse, Error<DeleteDestinationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/notifications/v1/destinations/{destinationId}", local_var_configuration.base_path, destinationId=crate::apis::urlencode(destination_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteDestinationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes the subscription indicated by the subscription identifier and notification type that you specify. The subscription identifier can be for any subscription associated with your application. After you successfully call this operation, notifications will stop being sent for the associated subscription. The deleteSubscriptionById API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn delete_subscription_by_id(configuration: &configuration::Configuration, subscription_id: &str, notification_type: &str) -> Result<crate::models::DeleteSubscriptionByIdResponse, Error<DeleteSubscriptionByIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/notifications/v1/subscriptions/{notificationType}/{subscriptionId}", local_var_configuration.base_path, subscriptionId=crate::apis::urlencode(subscription_id), notificationType=crate::apis::urlencode(notification_type));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteSubscriptionByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns information about the destination that you specify. The getDestination API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_destination(configuration: &configuration::Configuration, destination_id: &str) -> Result<crate::models::GetDestinationResponse, Error<GetDestinationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/notifications/v1/destinations/{destinationId}", local_var_configuration.base_path, destinationId=crate::apis::urlencode(destination_id));
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
        let local_var_entity: Option<GetDestinationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns information about all destinations. The getDestinations API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_destinations(configuration: &configuration::Configuration, ) -> Result<crate::models::GetDestinationsResponse, Error<GetDestinationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/notifications/v1/destinations", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetDestinationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns information about subscriptions of the specified notification type. You can use this API to get subscription information when you do not have a subscription identifier.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_subscription(configuration: &configuration::Configuration, notification_type: &str) -> Result<crate::models::GetSubscriptionResponse, Error<GetSubscriptionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/notifications/v1/subscriptions/{notificationType}", local_var_configuration.base_path, notificationType=crate::apis::urlencode(notification_type));
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
        let local_var_entity: Option<GetSubscriptionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns information about a subscription for the specified notification type. The getSubscriptionById API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_subscription_by_id(configuration: &configuration::Configuration, subscription_id: &str, notification_type: &str) -> Result<crate::models::GetSubscriptionByIdResponse, Error<GetSubscriptionByIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/notifications/v1/subscriptions/{notificationType}/{subscriptionId}", local_var_configuration.base_path, subscriptionId=crate::apis::urlencode(subscription_id), notificationType=crate::apis::urlencode(notification_type));
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
        let local_var_entity: Option<GetSubscriptionByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

