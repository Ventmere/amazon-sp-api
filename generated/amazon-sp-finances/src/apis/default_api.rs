/*
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use super::{Error, configuration};
use amazon_sp_api_shared::{request::UrlBuilder, error::ResponseError};


/// Returns financial event groups for a given date range.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn list_financial_event_groups(configuration: &configuration::Configuration, max_results_per_page: Option<i32>, financial_event_group_started_before: Option<String>, financial_event_group_started_after: Option<String>, next_token: Option<&str>) -> Result<crate::models::ListFinancialEventGroupsResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/finances/v0/financialEventGroups", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = max_results_per_page {
        url_builder = url_builder.query(&[("MaxResultsPerPage", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = financial_event_group_started_before {
        url_builder = url_builder.query(&[("FinancialEventGroupStartedBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = financial_event_group_started_after {
        url_builder = url_builder.query(&[("FinancialEventGroupStartedAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        url_builder = url_builder.query(&[("NextToken", &local_var_str.to_string())]);
    }

    let url = url_builder.build()?;
    let access_token = if let Some(ref rdt) = local_var_configuration.rdt {
        Some(rdt.token()?)
    } else {
        if let Some(ref auth) = local_var_configuration.auth {
            Some(auth.get_access_token(&local_var_configuration.client).await?)
        } else {
            None
        }
    };

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
	    url.as_str(),
	    "GET",
        access_token.clone(),
	    &"",
	    ) {
	      Ok(new_headers) => new_headers,
	      Err(err) => return Err(Error::AWSV4SignatureError(err)),
	    };
	for (local_var_name, local_var_value) in local_var_new_headers.iter() {
	    local_var_req_builder = local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
	}
    }

    if let Some(token) = access_token {
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
        let error_list = serde_json::from_str::<amazon_sp_api_shared::request::ErrorList>(&local_var_content).ok();
        let local_var_error = ResponseError { status: local_var_status, content: local_var_content, error_list: error_list.map(|e| e.errors) };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns financial events for the specified data range.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn list_financial_events(configuration: &configuration::Configuration, max_results_per_page: Option<i32>, posted_after: Option<String>, posted_before: Option<String>, next_token: Option<&str>) -> Result<crate::models::ListFinancialEventsResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/finances/v0/financialEvents", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = max_results_per_page {
        url_builder = url_builder.query(&[("MaxResultsPerPage", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = posted_after {
        url_builder = url_builder.query(&[("PostedAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = posted_before {
        url_builder = url_builder.query(&[("PostedBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        url_builder = url_builder.query(&[("NextToken", &local_var_str.to_string())]);
    }

    let url = url_builder.build()?;
    let access_token = if let Some(ref rdt) = local_var_configuration.rdt {
        Some(rdt.token()?)
    } else {
        if let Some(ref auth) = local_var_configuration.auth {
            Some(auth.get_access_token(&local_var_configuration.client).await?)
        } else {
            None
        }
    };

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
	    url.as_str(),
	    "GET",
        access_token.clone(),
	    &"",
	    ) {
	      Ok(new_headers) => new_headers,
	      Err(err) => return Err(Error::AWSV4SignatureError(err)),
	    };
	for (local_var_name, local_var_value) in local_var_new_headers.iter() {
	    local_var_req_builder = local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
	}
    }

    if let Some(token) = access_token {
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
        let error_list = serde_json::from_str::<amazon_sp_api_shared::request::ErrorList>(&local_var_content).ok();
        let local_var_error = ResponseError { status: local_var_status, content: local_var_content, error_list: error_list.map(|e| e.errors) };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all financial events for the specified financial event group.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn list_financial_events_by_group_id(configuration: &configuration::Configuration, event_group_id: &str, max_results_per_page: Option<i32>, next_token: Option<&str>) -> Result<crate::models::ListFinancialEventsResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/finances/v0/financialEventGroups/{eventGroupId}/financialEvents", local_var_configuration.base_path, eventGroupId=crate::apis::urlencode(event_group_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = max_results_per_page {
        url_builder = url_builder.query(&[("MaxResultsPerPage", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        url_builder = url_builder.query(&[("NextToken", &local_var_str.to_string())]);
    }

    let url = url_builder.build()?;
    let access_token = if let Some(ref rdt) = local_var_configuration.rdt {
        Some(rdt.token()?)
    } else {
        if let Some(ref auth) = local_var_configuration.auth {
            Some(auth.get_access_token(&local_var_configuration.client).await?)
        } else {
            None
        }
    };

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
	    url.as_str(),
	    "GET",
        access_token.clone(),
	    &"",
	    ) {
	      Ok(new_headers) => new_headers,
	      Err(err) => return Err(Error::AWSV4SignatureError(err)),
	    };
	for (local_var_name, local_var_value) in local_var_new_headers.iter() {
	    local_var_req_builder = local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
	}
    }

    if let Some(token) = access_token {
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
        let error_list = serde_json::from_str::<amazon_sp_api_shared::request::ErrorList>(&local_var_content).ok();
        let local_var_error = ResponseError { status: local_var_status, content: local_var_content, error_list: error_list.map(|e| e.errors) };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all financial events for the specified order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn list_financial_events_by_order_id(configuration: &configuration::Configuration, order_id: &str, max_results_per_page: Option<i32>, next_token: Option<&str>) -> Result<crate::models::ListFinancialEventsResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/finances/v0/orders/{orderId}/financialEvents", local_var_configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = max_results_per_page {
        url_builder = url_builder.query(&[("MaxResultsPerPage", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        url_builder = url_builder.query(&[("NextToken", &local_var_str.to_string())]);
    }

    let url = url_builder.build()?;
    let access_token = if let Some(ref rdt) = local_var_configuration.rdt {
        Some(rdt.token()?)
    } else {
        if let Some(ref auth) = local_var_configuration.auth {
            Some(auth.get_access_token(&local_var_configuration.client).await?)
        } else {
            None
        }
    };

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
	    url.as_str(),
	    "GET",
        access_token.clone(),
	    &"",
	    ) {
	      Ok(new_headers) => new_headers,
	      Err(err) => return Err(Error::AWSV4SignatureError(err)),
	    };
	for (local_var_name, local_var_value) in local_var_new_headers.iter() {
	    local_var_req_builder = local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
	}
    }

    if let Some(token) = access_token {
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
        let error_list = serde_json::from_str::<amazon_sp_api_shared::request::ErrorList>(&local_var_content).ok();
        let local_var_error = ResponseError { status: local_var_status, content: local_var_content, error_list: error_list.map(|e| e.errors) };
        Err(Error::ResponseError(local_var_error))
    }
}

