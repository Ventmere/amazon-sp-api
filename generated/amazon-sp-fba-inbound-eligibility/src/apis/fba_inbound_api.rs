/*
 * Selling Partner API for FBA Inbound Eligibilty
 *
 * With the FBA Inbound Eligibility API, you can build applications that let sellers get eligibility previews for items before shipping them to Amazon's fulfillment centers. With this API you can find out if an item is eligible for inbound shipment to Amazon's fulfillment centers in a specific marketplace. You can also find out if an item is eligible for using the manufacturer barcode for FBA inventory tracking. Sellers can use this information to inform their decisions about which items to ship Amazon's fulfillment centers.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use super::{Error, configuration};
use amazon_sp_api_shared::{request::UrlBuilder, error::ResponseError};


/// This operation gets an eligibility preview for an item that you specify. You can specify the type of eligibility preview that you want (INBOUND or COMMINGLING). For INBOUND previews, you can specify the marketplace in which you want to determine the item's eligibility.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_item_eligibility_preview(configuration: &configuration::Configuration, asin: &str, program: &str, marketplace_ids: Option<Vec<String>>) -> Result<crate::models::GetItemEligibilityPreviewResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/fba/inbound/v1/eligibility/itemPreview", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    if let Some(ref local_var_str) = marketplace_ids {
        url_builder = match "csv" {
            "multi" => url_builder.query(&local_var_str.into_iter().map(|p| ("marketplaceIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => url_builder.query(&[("marketplaceIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    url_builder = url_builder.query(&[("asin", &asin.to_string())]);
    url_builder = url_builder.query(&[("program", &program.to_string())]);

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

