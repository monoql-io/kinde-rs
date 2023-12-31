/*
 * Kinde Management API
 *
 * Provides endpoints to manage your Kinde Businesses
 *
 * The version of the OpenAPI document: 1
 * Contact: support@kinde.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`add_logout_redirect_urls`]
#[derive(Clone, Debug)]
pub struct AddLogoutRedirectUrlsParams {
    /// The identifier for the application.
    pub app_id: String,
    /// Callback details.
    pub replace_logout_redirect_urls_request: crate::models::ReplaceLogoutRedirectUrlsRequest
}

/// struct for passing parameters to the method [`add_redirect_callback_urls`]
#[derive(Clone, Debug)]
pub struct AddRedirectCallbackUrlsParams {
    /// The identifier for the application.
    pub app_id: String,
    /// Callback details.
    pub replace_redirect_callback_urls_request: crate::models::ReplaceRedirectCallbackUrlsRequest
}

/// struct for passing parameters to the method [`delete_callback_urls`]
#[derive(Clone, Debug)]
pub struct DeleteCallbackUrlsParams {
    /// The identifier for the application.
    pub app_id: String,
    /// Urls to delete, comma separated and url encoded.
    pub urls: String
}

/// struct for passing parameters to the method [`delete_logout_urls`]
#[derive(Clone, Debug)]
pub struct DeleteLogoutUrlsParams {
    /// The identifier for the application.
    pub app_id: String,
    /// Urls to delete, comma separated and url encoded.
    pub urls: String
}

/// struct for passing parameters to the method [`get_callback_urls`]
#[derive(Clone, Debug)]
pub struct GetCallbackUrlsParams {
    /// The identifier for the application.
    pub app_id: String
}

/// struct for passing parameters to the method [`get_logout_urls`]
#[derive(Clone, Debug)]
pub struct GetLogoutUrlsParams {
    /// The identifier for the application.
    pub app_id: String
}

/// struct for passing parameters to the method [`replace_logout_redirect_urls`]
#[derive(Clone, Debug)]
pub struct ReplaceLogoutRedirectUrlsParams {
    /// The identifier for the application.
    pub app_id: String,
    /// Callback details.
    pub replace_logout_redirect_urls_request: crate::models::ReplaceLogoutRedirectUrlsRequest
}

/// struct for passing parameters to the method [`replace_redirect_callback_urls`]
#[derive(Clone, Debug)]
pub struct ReplaceRedirectCallbackUrlsParams {
    /// The identifier for the application.
    pub app_id: String,
    /// Callback details.
    pub replace_redirect_callback_urls_request: crate::models::ReplaceRedirectCallbackUrlsRequest
}


/// struct for typed errors of method [`add_logout_redirect_urls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddLogoutRedirectUrlsError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_redirect_callback_urls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddRedirectCallbackUrlsError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_callback_urls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCallbackUrlsError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_logout_urls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogoutUrlsError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_callback_urls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCallbackUrlsError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_logout_urls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogoutUrlsError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_logout_redirect_urls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceLogoutRedirectUrlsError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_redirect_callback_urls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceRedirectCallbackUrlsError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}


/// Add additional logout redirect URLs. 
pub async fn add_logout_redirect_urls(configuration: &configuration::Configuration, params: AddLogoutRedirectUrlsParams) -> Result<crate::models::SuccessResponse, Error<AddLogoutRedirectUrlsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let app_id = params.app_id;
    let replace_logout_redirect_urls_request = params.replace_logout_redirect_urls_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{app_id}/auth_logout_urls", local_var_configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&replace_logout_redirect_urls_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddLogoutRedirectUrlsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Add additional redirect callback URLs. 
pub async fn add_redirect_callback_urls(configuration: &configuration::Configuration, params: AddRedirectCallbackUrlsParams) -> Result<crate::models::SuccessResponse, Error<AddRedirectCallbackUrlsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let app_id = params.app_id;
    let replace_redirect_callback_urls_request = params.replace_redirect_callback_urls_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{app_id}/auth_redirect_urls", local_var_configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&replace_redirect_callback_urls_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddRedirectCallbackUrlsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete callback URLs. 
pub async fn delete_callback_urls(configuration: &configuration::Configuration, params: DeleteCallbackUrlsParams) -> Result<crate::models::SuccessResponse, Error<DeleteCallbackUrlsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let app_id = params.app_id;
    let urls = params.urls;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{app_id}/auth_redirect_urls", local_var_configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("urls", &urls.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteCallbackUrlsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete logout URLs. 
pub async fn delete_logout_urls(configuration: &configuration::Configuration, params: DeleteLogoutUrlsParams) -> Result<crate::models::SuccessResponse, Error<DeleteLogoutUrlsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let app_id = params.app_id;
    let urls = params.urls;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{app_id}/auth_logout_urls", local_var_configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("urls", &urls.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteLogoutUrlsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an application's redirect callback URLs. 
pub async fn get_callback_urls(configuration: &configuration::Configuration, params: GetCallbackUrlsParams) -> Result<crate::models::RedirectCallbackUrls, Error<GetCallbackUrlsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let app_id = params.app_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{app_id}/auth_redirect_urls", local_var_configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCallbackUrlsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an application's logout redirect URLs. 
pub async fn get_logout_urls(configuration: &configuration::Configuration, params: GetLogoutUrlsParams) -> Result<crate::models::LogoutRedirectUrls, Error<GetLogoutUrlsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let app_id = params.app_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{app_id}/auth_logout_urls", local_var_configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLogoutUrlsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replace all logout redirect URLs. 
pub async fn replace_logout_redirect_urls(configuration: &configuration::Configuration, params: ReplaceLogoutRedirectUrlsParams) -> Result<crate::models::SuccessResponse, Error<ReplaceLogoutRedirectUrlsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let app_id = params.app_id;
    let replace_logout_redirect_urls_request = params.replace_logout_redirect_urls_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{app_id}/auth_logout_urls", local_var_configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&replace_logout_redirect_urls_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceLogoutRedirectUrlsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replace all redirect callback URLs. 
pub async fn replace_redirect_callback_urls(configuration: &configuration::Configuration, params: ReplaceRedirectCallbackUrlsParams) -> Result<crate::models::SuccessResponse, Error<ReplaceRedirectCallbackUrlsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let app_id = params.app_id;
    let replace_redirect_callback_urls_request = params.replace_redirect_callback_urls_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{app_id}/auth_redirect_urls", local_var_configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&replace_redirect_callback_urls_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceRedirectCallbackUrlsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

