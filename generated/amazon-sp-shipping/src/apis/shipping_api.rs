/*
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.   **Note:** If you are new to the Amazon Shipping API, refer to the latest version of <a href=\"https://developer-docs.amazon.com/amazon-shipping/docs/shipping-api-v2-reference\">Amazon Shipping API (v2)</a> on the <a href=\"https://developer-docs.amazon.com/amazon-shipping/\">Amazon Shipping Developer Documentation</a> site.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use super::{Error, configuration};
use amazon_sp_api_shared::{request::UrlBuilder, error::ResponseError};


/// Cancel a shipment by the given shipmentId.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn cancel_shipment(configuration: &configuration::Configuration, shipment_id: &str) -> Result<crate::models::CancelShipmentResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/shipping/v1/shipments/{shipmentId}/cancel", local_var_configuration.base_path, shipmentId=crate::apis::urlencode(shipment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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

/// Create a new shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 15 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
pub async fn create_shipment(configuration: &configuration::Configuration, body: crate::models::CreateShipmentRequest) -> Result<crate::models::CreateShipmentResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/shipping/v1/shipments", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
	    &serde_json::to_string(&body).expect("param should serialize to string"),
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
    local_var_req_builder = local_var_req_builder.json(&body);

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

/// Verify if the current account is valid.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_account(configuration: &configuration::Configuration, ) -> Result<crate::models::GetAccountResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/shipping/v1/account", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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

/// Get service rates.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_rates(configuration: &configuration::Configuration, body: crate::models::GetRatesRequest) -> Result<crate::models::GetRatesResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/shipping/v1/rates", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
	    &serde_json::to_string(&body).expect("param should serialize to string"),
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
    local_var_req_builder = local_var_req_builder.json(&body);

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

/// Return the entire shipment object for the shipmentId.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_shipment(configuration: &configuration::Configuration, shipment_id: &str) -> Result<crate::models::GetShipmentResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/shipping/v1/shipments/{shipmentId}", local_var_configuration.base_path, shipmentId=crate::apis::urlencode(shipment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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

/// Return the tracking information of a shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn get_tracking_information(configuration: &configuration::Configuration, tracking_id: &str) -> Result<crate::models::GetTrackingInformationResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/shipping/v1/tracking/{trackingId}", local_var_configuration.base_path, trackingId=crate::apis::urlencode(tracking_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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

/// Purchase shipping labels based on a given rate.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn purchase_labels(configuration: &configuration::Configuration, shipment_id: &str, body: crate::models::PurchaseLabelsRequest) -> Result<crate::models::PurchaseLabelsResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/shipping/v1/shipments/{shipmentId}/purchaseLabels", local_var_configuration.base_path, shipmentId=crate::apis::urlencode(shipment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
	    &serde_json::to_string(&body).expect("param should serialize to string"),
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
    local_var_req_builder = local_var_req_builder.json(&body);

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

/// Purchase shipping labels.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn purchase_shipment(configuration: &configuration::Configuration, body: crate::models::PurchaseShipmentRequest) -> Result<crate::models::PurchaseShipmentResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/shipping/v1/purchaseShipment", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
	    &serde_json::to_string(&body).expect("param should serialize to string"),
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
    local_var_req_builder = local_var_req_builder.json(&body);

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

/// Retrieve shipping label based on the shipment id and tracking id.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
pub async fn retrieve_shipping_label(configuration: &configuration::Configuration, shipment_id: &str, tracking_id: &str, body: crate::models::RetrieveShippingLabelRequest) -> Result<crate::models::RetrieveShippingLabelResponse, Error> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;


    let local_var_uri_str = format!("{}/shipping/v1/shipments/{shipmentId}/containers/{trackingId}/label", local_var_configuration.base_path, shipmentId=crate::apis::urlencode(shipment_id), trackingId=crate::apis::urlencode(tracking_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());
    #[allow(unused_mut)]
    let mut url_builder = UrlBuilder::parse(&local_var_uri_str)?;


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
	    &serde_json::to_string(&body).expect("param should serialize to string"),
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
    local_var_req_builder = local_var_req_builder.json(&body);

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

