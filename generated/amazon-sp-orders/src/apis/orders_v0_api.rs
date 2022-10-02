/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};
use amazon_sp_api_shared::request::UrlBuilder;


/// struct for typed errors of method [`get_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderError {
    Status400(crate::models::GetOrderResponse),
    Status403(crate::models::GetOrderResponse),
    Status404(crate::models::GetOrderResponse),
    Status429(crate::models::GetOrderResponse),
    Status500(crate::models::GetOrderResponse),
    Status503(crate::models::GetOrderResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_order_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderAddressError {
    Status400(crate::models::GetOrderAddressResponse),
    Status403(crate::models::GetOrderAddressResponse),
    Status404(crate::models::GetOrderAddressResponse),
    Status429(crate::models::GetOrderAddressResponse),
    Status500(crate::models::GetOrderAddressResponse),
    Status503(crate::models::GetOrderAddressResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_order_buyer_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderBuyerInfoError {
    Status400(crate::models::GetOrderBuyerInfoResponse),
    Status403(crate::models::GetOrderBuyerInfoResponse),
    Status404(crate::models::GetOrderBuyerInfoResponse),
    Status429(crate::models::GetOrderBuyerInfoResponse),
    Status500(crate::models::GetOrderBuyerInfoResponse),
    Status503(crate::models::GetOrderBuyerInfoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_order_items`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderItemsError {
    Status400(crate::models::GetOrderItemsResponse),
    Status403(crate::models::GetOrderItemsResponse),
    Status404(crate::models::GetOrderItemsResponse),
    Status429(crate::models::GetOrderItemsResponse),
    Status500(crate::models::GetOrderItemsResponse),
    Status503(crate::models::GetOrderItemsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_order_items_buyer_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderItemsBuyerInfoError {
    Status400(crate::models::GetOrderItemsBuyerInfoResponse),
    Status403(crate::models::GetOrderItemsBuyerInfoResponse),
    Status404(crate::models::GetOrderItemsBuyerInfoResponse),
    Status429(crate::models::GetOrderItemsBuyerInfoResponse),
    Status500(crate::models::GetOrderItemsBuyerInfoResponse),
    Status503(crate::models::GetOrderItemsBuyerInfoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_order_regulated_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderRegulatedInfoError {
    Status400(crate::models::GetOrderRegulatedInfoResponse),
    Status403(crate::models::GetOrderRegulatedInfoResponse),
    Status404(crate::models::GetOrderRegulatedInfoResponse),
    Status429(crate::models::GetOrderRegulatedInfoResponse),
    Status500(crate::models::GetOrderRegulatedInfoResponse),
    Status503(crate::models::GetOrderRegulatedInfoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_orders`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrdersError {
    Status400(crate::models::GetOrdersResponse),
    Status403(crate::models::GetOrdersResponse),
    Status404(crate::models::GetOrdersResponse),
    Status429(crate::models::GetOrdersResponse),
    Status500(crate::models::GetOrdersResponse),
    Status503(crate::models::GetOrdersResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_verification_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateVerificationStatusError {
    Status400(crate::models::UpdateVerificationStatusErrorResponse),
    Status403(crate::models::UpdateVerificationStatusErrorResponse),
    Status404(crate::models::UpdateVerificationStatusErrorResponse),
    Status413(crate::models::UpdateVerificationStatusErrorResponse),
    Status415(crate::models::UpdateVerificationStatusErrorResponse),
    Status429(crate::models::UpdateVerificationStatusErrorResponse),
    Status500(crate::models::UpdateVerificationStatusErrorResponse),
    Status503(crate::models::UpdateVerificationStatusErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Returns the order that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0167 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_order(configuration: &configuration::Configuration, order_id: &str) -> Result<crate::models::GetOrderResponse, Error<GetOrderError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/orders/v0/orders/{orderId}", local_var_configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    let url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
        let local_var_entity: Option<GetOrderError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the shipping address for the order that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0167 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_order_address(configuration: &configuration::Configuration, order_id: &str) -> Result<crate::models::GetOrderAddressResponse, Error<GetOrderAddressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/orders/v0/orders/{orderId}/address", local_var_configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    let url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
        let local_var_entity: Option<GetOrderAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns buyer information for the order that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0167 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_order_buyer_info(configuration: &configuration::Configuration, order_id: &str) -> Result<crate::models::GetOrderBuyerInfoResponse, Error<GetOrderBuyerInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/orders/v0/orders/{orderId}/buyerInfo", local_var_configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    let url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
        let local_var_entity: Option<GetOrderBuyerInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns detailed order item information for the order that you specify. If NextToken is provided, it's used to retrieve the next page of order items.  __Note__: When an order is in the Pending state (the order has been placed but payment has not been authorized), the getOrderItems operation does not return information about pricing, taxes, shipping charges, gift status or promotions for the order items in the order. After an order leaves the Pending state (this occurs when payment has been authorized) and enters the Unshipped, Partially Shipped, or Shipped state, the getOrderItems operation returns information about pricing, taxes, shipping charges, gift status and promotions for the order items in the order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_order_items(configuration: &configuration::Configuration, order_id: &str, next_token: Option<&str>) -> Result<crate::models::GetOrderItemsResponse, Error<GetOrderItemsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/orders/v0/orders/{orderId}/orderItems", local_var_configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    let url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = next_token {
        url_builder = url_builder.query(&[("NextToken", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetOrderItemsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns buyer information for the order items in the order that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_order_items_buyer_info(configuration: &configuration::Configuration, order_id: &str, next_token: Option<&str>) -> Result<crate::models::GetOrderItemsBuyerInfoResponse, Error<GetOrderItemsBuyerInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/orders/v0/orders/{orderId}/orderItems/buyerInfo", local_var_configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    let url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = next_token {
        url_builder = url_builder.query(&[("NextToken", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetOrderItemsBuyerInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns regulated information for the order that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_order_regulated_info(configuration: &configuration::Configuration, order_id: &str) -> Result<crate::models::GetOrderRegulatedInfoResponse, Error<GetOrderRegulatedInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/orders/v0/orders/{orderId}/regulatedInfo", local_var_configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    let url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
        let local_var_entity: Option<GetOrderRegulatedInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns orders created or updated during the time frame indicated by the specified parameters. You can also apply a range of filtering criteria to narrow the list of orders returned. If NextToken is present, that will be used to retrieve the orders instead of other criteria.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0167 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_orders(configuration: &configuration::Configuration, marketplace_ids: Vec<String>, created_after: Option<&str>, created_before: Option<&str>, last_updated_after: Option<&str>, last_updated_before: Option<&str>, order_statuses: Option<Vec<String>>, fulfillment_channels: Option<Vec<String>>, payment_methods: Option<Vec<String>>, buyer_email: Option<&str>, seller_order_id: Option<&str>, max_results_per_page: Option<i32>, easy_ship_shipment_statuses: Option<Vec<String>>, electronic_invoice_statuses: Option<Vec<String>>, next_token: Option<&str>, amazon_order_ids: Option<Vec<String>>, actual_fulfillment_supply_source_id: Option<&str>, is_ispu: Option<bool>, store_chain_store_id: Option<&str>) -> Result<crate::models::GetOrdersResponse, Error<GetOrdersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/orders/v0/orders", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    let url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = created_after {
        url_builder = url_builder.query(&[("CreatedAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_before {
        url_builder = url_builder.query(&[("CreatedBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = last_updated_after {
        url_builder = url_builder.query(&[("LastUpdatedAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = last_updated_before {
        url_builder = url_builder.query(&[("LastUpdatedBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order_statuses {
        url_builder = match "csv" {
            "multi" => url_builder.query(&local_var_str.into_iter().map(|p| ("OrderStatuses".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => url_builder.query(&[("OrderStatuses", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    url_builder = match "csv" {
        "multi" => url_builder.query(&marketplace_ids.into_iter().map(|p| ("MarketplaceIds".to_owned(), p)).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => url_builder.query(&[("MarketplaceIds", &marketplace_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    if let Some(ref local_var_str) = fulfillment_channels {
        url_builder = match "csv" {
            "multi" => url_builder.query(&local_var_str.into_iter().map(|p| ("FulfillmentChannels".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => url_builder.query(&[("FulfillmentChannels", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = payment_methods {
        url_builder = match "csv" {
            "multi" => url_builder.query(&local_var_str.into_iter().map(|p| ("PaymentMethods".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => url_builder.query(&[("PaymentMethods", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = buyer_email {
        url_builder = url_builder.query(&[("BuyerEmail", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = seller_order_id {
        url_builder = url_builder.query(&[("SellerOrderId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_results_per_page {
        url_builder = url_builder.query(&[("MaxResultsPerPage", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = easy_ship_shipment_statuses {
        url_builder = match "csv" {
            "multi" => url_builder.query(&local_var_str.into_iter().map(|p| ("EasyShipShipmentStatuses".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => url_builder.query(&[("EasyShipShipmentStatuses", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = electronic_invoice_statuses {
        url_builder = match "csv" {
            "multi" => url_builder.query(&local_var_str.into_iter().map(|p| ("ElectronicInvoiceStatuses".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => url_builder.query(&[("ElectronicInvoiceStatuses", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = next_token {
        url_builder = url_builder.query(&[("NextToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = amazon_order_ids {
        url_builder = match "csv" {
            "multi" => url_builder.query(&local_var_str.into_iter().map(|p| ("AmazonOrderIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => url_builder.query(&[("AmazonOrderIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = actual_fulfillment_supply_source_id {
        url_builder = url_builder.query(&[("ActualFulfillmentSupplySourceId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_ispu {
        url_builder = url_builder.query(&[("IsISPU", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = store_chain_store_id {
        url_builder = url_builder.query(&[("StoreChainStoreId", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetOrdersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates (approves or rejects) the verification status of an order containing regulated products.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn update_verification_status(configuration: &configuration::Configuration, order_id: &str, payload: crate::models::UpdateVerificationStatusRequest) -> Result<(), Error<UpdateVerificationStatusError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/orders/v0/orders/{orderId}/regulatedInfo", local_var_configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());
    let url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
	    &serde_json::to_string(&payload).expect("param should serialize to string"),
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
    local_var_req_builder = local_var_req_builder.json(&payload);

    let mut local_var_req = local_var_req_builder.build()?;
    *local_var_req.url_mut() = url;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateVerificationStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

