/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};
use amazon_sp_api_shared::request::UrlBuilder;


/// struct for typed errors of method [`create_scheduled_package`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScheduledPackageError {
    Status400(crate::models::ErrorList),
    Status401(crate::models::ErrorList),
    Status403(crate::models::ErrorList),
    Status404(crate::models::ErrorList),
    Status415(crate::models::ErrorList),
    Status429(crate::models::ErrorList),
    Status500(crate::models::ErrorList),
    Status503(crate::models::ErrorList),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_scheduled_package_bulk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScheduledPackageBulkError {
    Status400(crate::models::ErrorList),
    Status401(crate::models::ErrorList),
    Status403(crate::models::ErrorList),
    Status404(crate::models::ErrorList),
    Status429(crate::models::ErrorList),
    Status415(crate::models::ErrorList),
    Status500(crate::models::ErrorList),
    Status503(crate::models::ErrorList),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_scheduled_package`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetScheduledPackageError {
    Status400(crate::models::ErrorList),
    Status401(crate::models::ErrorList),
    Status403(crate::models::ErrorList),
    Status404(crate::models::ErrorList),
    Status415(crate::models::ErrorList),
    Status429(crate::models::ErrorList),
    Status500(crate::models::ErrorList),
    Status503(crate::models::ErrorList),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_handover_slots`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHandoverSlotsError {
    Status400(crate::models::ErrorList),
    Status401(crate::models::ErrorList),
    Status403(crate::models::ErrorList),
    Status404(crate::models::ErrorList),
    Status415(crate::models::ErrorList),
    Status429(crate::models::ErrorList),
    Status500(crate::models::ErrorList),
    Status503(crate::models::ErrorList),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_scheduled_packages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateScheduledPackagesError {
    Status400(crate::models::ErrorList),
    Status401(crate::models::ErrorList),
    Status403(crate::models::ErrorList),
    Status404(crate::models::ErrorList),
    Status415(crate::models::ErrorList),
    Status429(crate::models::ErrorList),
    Status500(crate::models::ErrorList),
    Status503(crate::models::ErrorList),
    UnknownValue(serde_json::Value),
}


/// Schedules an Easy Ship order and returns the scheduled package information.  This operation does the following:  *  Specifies the time slot and handover method for the order to be scheduled for delivery.  * Updates the Easy Ship order status.  * Generates a shipping label and an invoice. Calling `createScheduledPackage` also generates a warranty document if you specify a `SerialNumber` value. To get these documents, see [How to get invoice, shipping label, and warranty documents](doc:easy-ship-api-v2022-03-23-use-case-guide).  * Shows the status of Easy Ship orders when you call the `getOrders` operation of the Selling Partner API for Orders and examine the `EasyShipShipmentStatus` property in the response body.  See the **Shipping Label**, **Invoice**, and **Warranty** columns in the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) to see which documents are supported in each marketplace.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn create_scheduled_package(configuration: &configuration::Configuration, create_scheduled_package_request: crate::models::CreateScheduledPackageRequest) -> Result<crate::models::Package, Error<CreateScheduledPackageError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/easyShip/2022-03-23/package", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


    let url = url_builder.build()?;

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
	    url.as_str(),
	    "POST",
        if let Some(ref auth) = configuration.auth {
            Some(auth.get_access_token(&configuration.client).await?)
        } else {
            None
        },
	    &serde_json::to_string(&create_scheduled_package_request).expect("param should serialize to string"),
	    ) {
	      Ok(new_headers) => new_headers,
	      Err(err) => return Err(Error::AWSV4SignatureError(err)),
	    };
	for (local_var_name, local_var_value) in local_var_new_headers.iter() {
	    local_var_req_builder = local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
	}
    }

    if let Some(ref auth) = local_var_configuration.auth {
        let token = auth.get_access_token(&local_var_configuration.client).await?;
        local_var_req_builder = local_var_req_builder.header("x-amz-access-token", token.as_str());
    }

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_scheduled_package_request);

    let mut local_var_req = local_var_req_builder.build()?;
    *local_var_req.url_mut() = url;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateScheduledPackageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This operation automatically schedules a time slot for all the `amazonOrderId`s given as input, generating the associated shipping labels, along with other compliance documents according to the marketplace (refer to the [marketplace document support table](doc:easy-ship-api-v2022-03-23-marketplace-document-support-table)).  Developers calling this operation may optionally assign a `packageDetails` object, allowing them to input a preferred time slot for each order in ther request. In this case, Amazon will try to schedule the respective packages using their optional settings. On the other hand, *i.e.*, if the time slot is not provided, Amazon will then pick the earliest time slot possible.   Regarding the shipping label's file format, external developers are able to choose between PDF or ZPL, and Amazon will create the label accordingly.  This operation returns an array composed of the scheduled packages, and a short-lived URL pointing to a zip file containing the generated shipping labels and the other documents enabled for your marketplace. If at least an order couldn't be scheduled, then Amazon adds the `rejectedOrders` list into the response, which contains an entry for each order we couldn't process. Each entry is composed of an error message describing the reason of the failure, so that sellers can take action.  The table below displays the supported request and burst maximum rates: | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 | The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn create_scheduled_package_bulk(configuration: &configuration::Configuration, create_scheduled_packages_request: crate::models::CreateScheduledPackagesRequest) -> Result<crate::models::CreateScheduledPackagesResponse, Error<CreateScheduledPackageBulkError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/easyShip/2022-03-23/packages/bulk", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


    let url = url_builder.build()?;

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
	    url.as_str(),
	    "POST",
        if let Some(ref auth) = configuration.auth {
            Some(auth.get_access_token(&configuration.client).await?)
        } else {
            None
        },
	    &serde_json::to_string(&create_scheduled_packages_request).expect("param should serialize to string"),
	    ) {
	      Ok(new_headers) => new_headers,
	      Err(err) => return Err(Error::AWSV4SignatureError(err)),
	    };
	for (local_var_name, local_var_value) in local_var_new_headers.iter() {
	    local_var_req_builder = local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
	}
    }

    if let Some(ref auth) = local_var_configuration.auth {
        let token = auth.get_access_token(&local_var_configuration.client).await?;
        local_var_req_builder = local_var_req_builder.header("x-amz-access-token", token.as_str());
    }

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_scheduled_packages_request);

    let mut local_var_req = local_var_req_builder.build()?;
    *local_var_req.url_mut() = url;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateScheduledPackageBulkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns information about a package, including dimensions, weight, time slot information for handover, invoice and item information, and status.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_scheduled_package(configuration: &configuration::Configuration, amazon_order_id: &str, marketplace_id: &str) -> Result<crate::models::Package, Error<GetScheduledPackageError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/easyShip/2022-03-23/package", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    url_builder = url_builder.query(&[("amazonOrderId", &amazon_order_id.to_string())]);
    url_builder = url_builder.query(&[("marketplaceId", &marketplace_id.to_string())]);

    let url = url_builder.build()?;

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
	    url.as_str(),
	    "GET",
        if let Some(ref auth) = configuration.auth {
            Some(auth.get_access_token(&configuration.client).await?)
        } else {
            None
        },
	    &"",
	    ) {
	      Ok(new_headers) => new_headers,
	      Err(err) => return Err(Error::AWSV4SignatureError(err)),
	    };
	for (local_var_name, local_var_value) in local_var_new_headers.iter() {
	    local_var_req_builder = local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
	}
    }

    if let Some(ref auth) = local_var_configuration.auth {
        let token = auth.get_access_token(&local_var_configuration.client).await?;
        local_var_req_builder = local_var_req_builder.header("x-amz-access-token", token.as_str());
    }

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let mut local_var_req = local_var_req_builder.build()?;
    *local_var_req.url_mut() = url;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetScheduledPackageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns time slots available for Easy Ship orders to be scheduled based on the package weight and dimensions that the seller specifies.  This operation is available for scheduled and unscheduled orders based on marketplace support. See **Get Time Slots** in the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table).  This operation can return time slots that have either pickup or drop-off handover methods - see **Supported Handover Methods** in the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn list_handover_slots(configuration: &configuration::Configuration, list_handover_slots_request: Option<crate::models::ListHandoverSlotsRequest>) -> Result<crate::models::ListHandoverSlotsResponse, Error<ListHandoverSlotsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/easyShip/2022-03-23/timeSlot", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


    let url = url_builder.build()?;

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
	    url.as_str(),
	    "POST",
        if let Some(ref auth) = configuration.auth {
            Some(auth.get_access_token(&configuration.client).await?)
        } else {
            None
        },
	    &serde_json::to_string(&list_handover_slots_request).expect("param should serialize to string"),
	    ) {
	      Ok(new_headers) => new_headers,
	      Err(err) => return Err(Error::AWSV4SignatureError(err)),
	    };
	for (local_var_name, local_var_value) in local_var_new_headers.iter() {
	    local_var_req_builder = local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
	}
    }

    if let Some(ref auth) = local_var_configuration.auth {
        let token = auth.get_access_token(&local_var_configuration.client).await?;
        local_var_req_builder = local_var_req_builder.header("x-amz-access-token", token.as_str());
    }

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&list_handover_slots_request);

    let mut local_var_req = local_var_req_builder.build()?;
    *local_var_req.url_mut() = url;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListHandoverSlotsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the time slot for handing over the package indicated by the specified `scheduledPackageId`. You can get the new `slotId` value for the time slot by calling the `listHandoverSlots` operation before making another `patch` call.  See the **Update Package** column in the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) to see which marketplaces this operation is supported in.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn update_scheduled_packages(configuration: &configuration::Configuration, update_scheduled_packages_request: Option<crate::models::UpdateScheduledPackagesRequest>) -> Result<crate::models::Packages, Error<UpdateScheduledPackagesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/easyShip/2022-03-23/package", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


    let url = url_builder.build()?;

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
	    url.as_str(),
	    "PATCH",
        if let Some(ref auth) = configuration.auth {
            Some(auth.get_access_token(&configuration.client).await?)
        } else {
            None
        },
	    &serde_json::to_string(&update_scheduled_packages_request).expect("param should serialize to string"),
	    ) {
	      Ok(new_headers) => new_headers,
	      Err(err) => return Err(Error::AWSV4SignatureError(err)),
	    };
	for (local_var_name, local_var_value) in local_var_new_headers.iter() {
	    local_var_req_builder = local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
	}
    }

    if let Some(ref auth) = local_var_configuration.auth {
        let token = auth.get_access_token(&local_var_configuration.client).await?;
        local_var_req_builder = local_var_req_builder.header("x-amz-access-token", token.as_str());
    }

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&update_scheduled_packages_request);

    let mut local_var_req = local_var_req_builder.build()?;
    *local_var_req.url_mut() = url;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateScheduledPackagesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
