/*
 * Selling Partner API for Uploads
 *
 * The Uploads API lets you upload files that you can programmatically access using other Selling Partner APIs, such as the A+ Content API and the Messaging API.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use super::{Error, configuration};
use amazon_sp_api_shared::{request::UrlBuilder, error::ResponseError};


/// Creates an upload destination, returning the information required to upload a file to the destination and to programmatically access the file.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | .1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn create_upload_destination_for_resource(configuration: &configuration::Configuration, marketplace_ids: Vec<String>, content_md5: &str, resource: &str, content_type: Option<&str>) -> Result<crate::models::CreateUploadDestinationResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/uploads/2020-11-01/uploadDestinations/{resource}", local_var_configuration.base_path, resource=crate::apis::urlencode(resource));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    url_builder = match "csv" {
        "multi" => url_builder.query(&marketplace_ids.into_iter().map(|p| ("marketplaceIds".to_owned(), p)).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => url_builder.query(&[("marketplaceIds", &marketplace_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    url_builder = url_builder.query(&[("contentMD5", &content_md5.to_string())]);
    if let Some(ref local_var_str) = content_type {
        url_builder = url_builder.query(&[("contentType", &local_var_str.to_string())]);
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
	    "POST",
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

