/*
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};
use amazon_sp_api_shared::request::UrlBuilder;


/// struct for typed errors of method [`get_purchase_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPurchaseOrderError {
    Status400(crate::models::GetPurchaseOrderResponse),
    Status401(crate::models::GetPurchaseOrderResponse),
    Status403(crate::models::GetPurchaseOrderResponse),
    Status404(crate::models::GetPurchaseOrderResponse),
    Status415(crate::models::GetPurchaseOrderResponse),
    Status429(crate::models::GetPurchaseOrderResponse),
    Status500(crate::models::GetPurchaseOrderResponse),
    Status503(crate::models::GetPurchaseOrderResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_purchase_orders`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPurchaseOrdersError {
    Status400(crate::models::GetPurchaseOrdersResponse),
    Status403(crate::models::GetPurchaseOrdersResponse),
    Status404(crate::models::GetPurchaseOrdersResponse),
    Status415(crate::models::GetPurchaseOrdersResponse),
    Status429(crate::models::GetPurchaseOrdersResponse),
    Status500(crate::models::GetPurchaseOrdersResponse),
    Status503(crate::models::GetPurchaseOrdersResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_purchase_orders_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPurchaseOrdersStatusError {
    Status400(crate::models::GetPurchaseOrdersStatusResponse),
    Status403(crate::models::GetPurchaseOrdersStatusResponse),
    Status404(crate::models::GetPurchaseOrdersStatusResponse),
    Status415(crate::models::GetPurchaseOrdersStatusResponse),
    Status429(crate::models::GetPurchaseOrdersStatusResponse),
    Status500(crate::models::GetPurchaseOrdersStatusResponse),
    Status503(crate::models::GetPurchaseOrdersStatusResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`submit_acknowledgement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitAcknowledgementError {
    Status400(crate::models::SubmitAcknowledgementResponse),
    Status403(crate::models::SubmitAcknowledgementResponse),
    Status404(crate::models::SubmitAcknowledgementResponse),
    Status413(crate::models::SubmitAcknowledgementResponse),
    Status415(crate::models::SubmitAcknowledgementResponse),
    Status429(crate::models::SubmitAcknowledgementResponse),
    Status500(crate::models::SubmitAcknowledgementResponse),
    Status503(crate::models::SubmitAcknowledgementResponse),
    UnknownValue(serde_json::Value),
}


/// Returns a purchase order based on the purchaseOrderNumber value that you specify.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_purchase_order(configuration: &configuration::Configuration, purchase_order_number: &str) -> Result<crate::models::GetPurchaseOrderResponse, Error<GetPurchaseOrderError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/vendor/orders/v1/purchaseOrders/{purchaseOrderNumber}", local_var_configuration.base_path, purchaseOrderNumber=crate::apis::urlencode(purchase_order_number));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
        let local_var_entity: Option<GetPurchaseOrderError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of purchase orders created or changed during the time frame that you specify. You define the time frame using the createdAfter, createdBefore, changedAfter and changedBefore parameters. The date range to search must not be more than 7 days. You can choose to get only the purchase order numbers by setting includeDetails to false. You can then use the getPurchaseOrder operation to receive details for a specific purchase order.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_purchase_orders(configuration: &configuration::Configuration, limit: Option<i64>, created_after: Option<String>, created_before: Option<String>, sort_order: Option<&str>, next_token: Option<&str>, include_details: Option<&str>, changed_after: Option<String>, changed_before: Option<String>, po_item_state: Option<&str>, is_po_changed: Option<&str>, purchase_order_state: Option<&str>, ordering_vendor_code: Option<&str>) -> Result<crate::models::GetPurchaseOrdersResponse, Error<GetPurchaseOrdersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/vendor/orders/v1/purchaseOrders", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = limit {
        url_builder = url_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_after {
        url_builder = url_builder.query(&[("createdAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_before {
        url_builder = url_builder.query(&[("createdBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_order {
        url_builder = url_builder.query(&[("sortOrder", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        url_builder = url_builder.query(&[("nextToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_details {
        url_builder = url_builder.query(&[("includeDetails", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = changed_after {
        url_builder = url_builder.query(&[("changedAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = changed_before {
        url_builder = url_builder.query(&[("changedBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = po_item_state {
        url_builder = url_builder.query(&[("poItemState", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_po_changed {
        url_builder = url_builder.query(&[("isPOChanged", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = purchase_order_state {
        url_builder = url_builder.query(&[("purchaseOrderState", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ordering_vendor_code {
        url_builder = url_builder.query(&[("orderingVendorCode", &local_var_str.to_string())]);
    }

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
        let local_var_entity: Option<GetPurchaseOrdersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns purchase order statuses based on the filters that you specify. Date range to search must not be more than 7 days. You can return a list of purchase order statuses using the available filters, or a single purchase order status by providing the purchase order number.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_purchase_orders_status(configuration: &configuration::Configuration, limit: Option<i64>, sort_order: Option<&str>, next_token: Option<&str>, created_after: Option<String>, created_before: Option<String>, updated_after: Option<String>, updated_before: Option<String>, purchase_order_number: Option<&str>, purchase_order_status: Option<&str>, item_confirmation_status: Option<&str>, item_receive_status: Option<&str>, ordering_vendor_code: Option<&str>, ship_to_party_id: Option<&str>) -> Result<crate::models::GetPurchaseOrdersStatusResponse, Error<GetPurchaseOrdersStatusError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/vendor/orders/v1/purchaseOrdersStatus", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = limit {
        url_builder = url_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_order {
        url_builder = url_builder.query(&[("sortOrder", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        url_builder = url_builder.query(&[("nextToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_after {
        url_builder = url_builder.query(&[("createdAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_before {
        url_builder = url_builder.query(&[("createdBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = updated_after {
        url_builder = url_builder.query(&[("updatedAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = updated_before {
        url_builder = url_builder.query(&[("updatedBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = purchase_order_number {
        url_builder = url_builder.query(&[("purchaseOrderNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = purchase_order_status {
        url_builder = url_builder.query(&[("purchaseOrderStatus", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = item_confirmation_status {
        url_builder = url_builder.query(&[("itemConfirmationStatus", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = item_receive_status {
        url_builder = url_builder.query(&[("itemReceiveStatus", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ordering_vendor_code {
        url_builder = url_builder.query(&[("orderingVendorCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ship_to_party_id {
        url_builder = url_builder.query(&[("shipToPartyId", &local_var_str.to_string())]);
    }

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
        let local_var_entity: Option<GetPurchaseOrdersStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Submits acknowledgements for one or more purchase orders.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn submit_acknowledgement(configuration: &configuration::Configuration, body: crate::models::SubmitAcknowledgementRequest) -> Result<crate::models::SubmitAcknowledgementResponse, Error<SubmitAcknowledgementError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/vendor/orders/v1/acknowledgements", local_var_configuration.base_path);
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
	    &serde_json::to_string(&body).expect("param should serialize to string"),
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
    local_var_req_builder = local_var_req_builder.json(&body);

    let mut local_var_req = local_var_req_builder.build()?;
    *local_var_req.url_mut() = url;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SubmitAcknowledgementError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

