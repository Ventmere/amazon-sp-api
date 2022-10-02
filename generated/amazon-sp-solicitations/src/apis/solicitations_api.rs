/*
 * Selling Partner API for Solicitations
 *
 * With the Solicitations API you can build applications that send non-critical solicitations to buyers. You can get a list of solicitation types that are available for an order that you specify, then call an operation that sends a solicitation to the buyer for that order. Buyers cannot respond to solicitations sent by this API, and these solicitations do not appear in the Messaging section of Seller Central or in the recipient's Message Center. The Solicitations API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};
use amazon_sp_api_shared::request::UrlBuilder;


/// struct for typed errors of method [`create_product_review_and_seller_feedback_solicitation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateProductReviewAndSellerFeedbackSolicitationError {
    Status400(crate::models::CreateProductReviewAndSellerFeedbackSolicitationResponse),
    Status403(crate::models::CreateProductReviewAndSellerFeedbackSolicitationResponse),
    Status404(crate::models::CreateProductReviewAndSellerFeedbackSolicitationResponse),
    Status413(crate::models::CreateProductReviewAndSellerFeedbackSolicitationResponse),
    Status415(crate::models::CreateProductReviewAndSellerFeedbackSolicitationResponse),
    Status429(crate::models::CreateProductReviewAndSellerFeedbackSolicitationResponse),
    Status500(crate::models::CreateProductReviewAndSellerFeedbackSolicitationResponse),
    Status503(crate::models::CreateProductReviewAndSellerFeedbackSolicitationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_solicitation_actions_for_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSolicitationActionsForOrderError {
    Status400(crate::models::GetSolicitationActionsForOrderResponse),
    Status403(crate::models::GetSolicitationActionsForOrderResponse),
    Status404(crate::models::GetSolicitationActionsForOrderResponse),
    Status413(crate::models::GetSolicitationActionsForOrderResponse),
    Status415(crate::models::GetSolicitationActionsForOrderResponse),
    Status429(crate::models::GetSolicitationActionsForOrderResponse),
    Status500(crate::models::GetSolicitationActionsForOrderResponse),
    Status503(crate::models::GetSolicitationActionsForOrderResponse),
    UnknownValue(serde_json::Value),
}


/// Sends a solicitation to a buyer asking for seller feedback and a product review for the specified order. Send only one productReviewAndSellerFeedback or free form proactive message per order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn create_product_review_and_seller_feedback_solicitation(configuration: &configuration::Configuration, amazon_order_id: &str, marketplace_ids: Vec<String>) -> Result<crate::models::CreateProductReviewAndSellerFeedbackSolicitationResponse, Error<CreateProductReviewAndSellerFeedbackSolicitationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/solicitations/v1/orders/{amazonOrderId}/solicitations/productReviewAndSellerFeedback", local_var_configuration.base_path, amazonOrderId=crate::apis::urlencode(amazon_order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    url_builder = match "csv" {
        "multi" => url_builder.query(&marketplace_ids.into_iter().map(|p| ("marketplaceIds".to_owned(), p)).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => url_builder.query(&[("marketplaceIds", &marketplace_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };

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
        let local_var_entity: Option<CreateProductReviewAndSellerFeedbackSolicitationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of solicitation types that are available for an order that you specify. A solicitation type is represented by an actions object, which contains a path and query parameter(s). You can use the path and parameter(s) to call an operation that sends a solicitation. Currently only the productReviewAndSellerFeedbackSolicitation solicitation type is available.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_solicitation_actions_for_order(configuration: &configuration::Configuration, amazon_order_id: &str, marketplace_ids: Vec<String>) -> Result<crate::models::GetSolicitationActionsForOrderResponse, Error<GetSolicitationActionsForOrderError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/solicitations/v1/orders/{amazonOrderId}", local_var_configuration.base_path, amazonOrderId=crate::apis::urlencode(amazon_order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;

    url_builder = match "csv" {
        "multi" => url_builder.query(&marketplace_ids.into_iter().map(|p| ("marketplaceIds".to_owned(), p)).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => url_builder.query(&[("marketplaceIds", &marketplace_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };

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
        let local_var_entity: Option<GetSolicitationActionsForOrderError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

