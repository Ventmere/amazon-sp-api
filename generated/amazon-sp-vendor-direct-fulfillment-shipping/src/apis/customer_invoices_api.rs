/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};
use amazon_sp_api_shared::request::UrlBuilder;


/// struct for typed errors of method [`get_customer_invoice`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCustomerInvoiceError {
    Status400(crate::models::GetCustomerInvoiceResponse),
    Status401(crate::models::GetCustomerInvoiceResponse),
    Status403(crate::models::GetCustomerInvoiceResponse),
    Status404(crate::models::GetCustomerInvoiceResponse),
    Status415(crate::models::GetCustomerInvoiceResponse),
    Status429(crate::models::GetCustomerInvoiceResponse),
    Status500(crate::models::GetCustomerInvoiceResponse),
    Status503(crate::models::GetCustomerInvoiceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_customer_invoices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCustomerInvoicesError {
    Status400(crate::models::GetCustomerInvoiceResponse),
    Status403(crate::models::GetCustomerInvoiceResponse),
    Status404(crate::models::GetCustomerInvoiceResponse),
    Status415(crate::models::GetCustomerInvoiceResponse),
    Status429(crate::models::GetCustomerInvoiceResponse),
    Status500(crate::models::GetCustomerInvoiceResponse),
    Status503(crate::models::GetCustomerInvoiceResponse),
    UnknownValue(serde_json::Value),
}


/// Returns a customer invoice based on the purchaseOrderNumber that you specify.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_customer_invoice(configuration: &configuration::Configuration, purchase_order_number: &str) -> Result<crate::models::GetCustomerInvoiceResponse, Error<GetCustomerInvoiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/vendor/directFulfillment/shipping/v1/customerInvoices/{purchaseOrderNumber}", local_var_configuration.base_path, purchaseOrderNumber=crate::apis::urlencode(purchase_order_number));
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
        let local_var_entity: Option<GetCustomerInvoiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of customer invoices created during a time frame that you specify. You define the  time frame using the createdAfter and createdBefore parameters. You must use both of these parameters. The date range to search must be no more than 7 days.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_customer_invoices(configuration: &configuration::Configuration, created_after: String, created_before: String, ship_from_party_id: Option<&str>, limit: Option<i32>, sort_order: Option<&str>, next_token: Option<&str>) -> Result<crate::models::GetCustomerInvoicesResponse, Error<GetCustomerInvoicesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/vendor/directFulfillment/shipping/v1/customerInvoices", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = ship_from_party_id {
        url_builder = url_builder.query(&[("shipFromPartyId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        url_builder = url_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    url_builder = url_builder.query(&[("createdAfter", &created_after.to_string())]);
    url_builder = url_builder.query(&[("createdBefore", &created_before.to_string())]);
    if let Some(ref local_var_str) = sort_order {
        url_builder = url_builder.query(&[("sortOrder", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        url_builder = url_builder.query(&[("nextToken", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetCustomerInvoicesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

