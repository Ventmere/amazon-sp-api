/*
 * Selling Partner API for Product Fees
 *
 * The Selling Partner API for Product Fees lets you programmatically retrieve estimated fees for a product. You can then account for those fees in your pricing.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_my_fees_estimate_for_asin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyFeesEstimateForAsinError {
    Status400(crate::models::GetMyFeesEstimateResponse),
    Status401(crate::models::GetMyFeesEstimateResponse),
    Status403(crate::models::GetMyFeesEstimateResponse),
    Status404(crate::models::GetMyFeesEstimateResponse),
    Status429(crate::models::GetMyFeesEstimateResponse),
    Status500(crate::models::GetMyFeesEstimateResponse),
    Status503(crate::models::GetMyFeesEstimateResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_fees_estimate_for_sku`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyFeesEstimateForSkuError {
    Status400(crate::models::GetMyFeesEstimateResponse),
    Status401(crate::models::GetMyFeesEstimateResponse),
    Status403(crate::models::GetMyFeesEstimateResponse),
    Status404(crate::models::GetMyFeesEstimateResponse),
    Status429(crate::models::GetMyFeesEstimateResponse),
    Status500(crate::models::GetMyFeesEstimateResponse),
    Status503(crate::models::GetMyFeesEstimateResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_fees_estimates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyFeesEstimatesError {
    Status400(crate::models::GetMyFeesEstimatesErrorList),
    Status401(crate::models::GetMyFeesEstimatesErrorList),
    Status403(crate::models::GetMyFeesEstimatesErrorList),
    Status404(crate::models::GetMyFeesEstimatesErrorList),
    Status429(crate::models::GetMyFeesEstimatesErrorList),
    Status500(crate::models::GetMyFeesEstimatesErrorList),
    Status503(crate::models::GetMyFeesEstimatesErrorList),
    UnknownValue(serde_json::Value),
}


/// Returns the estimated fees for the item indicated by the specified ASIN in the marketplace specified in the request body.  You can call `getMyFeesEstimateForASIN` for an item on behalf of a selling partner before the selling partner sets the item's price. The selling partner can then take estimated fees into account. Each fees request must include an original identifier. This identifier is included in the fees estimate so you can correlate a fees estimate with the original request.  **Note:** This identifier value is only an estimate, actual costs may vary. Search \"fees\" in [Seller Central](https://sellercentral.amazon.com/) and consult the store-specific fee schedule for the most up-to-date information.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_my_fees_estimate_for_asin(configuration: &configuration::Configuration, asin: &str, body: crate::models::GetMyFeesEstimateRequest) -> Result<crate::models::GetMyFeesEstimateResponse, Error<GetMyFeesEstimateForAsinError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/products/fees/v0/items/{Asin}/feesEstimate", local_var_configuration.base_path, Asin=crate::apis::urlencode(asin));
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
        let local_var_entity: Option<GetMyFeesEstimateForAsinError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the estimated fees for the item indicated by the specified seller SKU in the marketplace specified in the request body.  You can call `getMyFeesEstimateForSKU` for an item on behalf of a selling partner before the selling partner sets the item's price. The selling partner can then take any estimated fees into account. Each fees estimate request must include an original identifier. This identifier is included in the fees estimate so that you can correlate a fees estimate with the original request.  **Note:** The identifier value is only an estimate, actual costs may vary. Search \"fees\" in [Seller Central](https://sellercentral.amazon.com/) and consult the store-specific fee schedule for the most up-to-date information.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_my_fees_estimate_for_sku(configuration: &configuration::Configuration, seller_sku: &str, body: crate::models::GetMyFeesEstimateRequest) -> Result<crate::models::GetMyFeesEstimateResponse, Error<GetMyFeesEstimateForSkuError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/products/fees/v0/listings/{SellerSKU}/feesEstimate", local_var_configuration.base_path, SellerSKU=crate::apis::urlencode(seller_sku));
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
        let local_var_entity: Option<GetMyFeesEstimateForSkuError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the estimated fees for a list of products.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn get_my_fees_estimates(configuration: &configuration::Configuration, body: Vec<crate::models::FeesEstimateByIdRequest>) -> Result<Vec<crate::models::FeesEstimateResult>, Error<GetMyFeesEstimatesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/products/fees/v0/feesEstimate", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetMyFeesEstimatesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

