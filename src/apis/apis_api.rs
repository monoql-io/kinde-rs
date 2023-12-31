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

/// struct for passing parameters to the method [`add_apis`]
#[derive(Clone, Debug)]
pub struct AddApisParams {
    /// API details.
    pub add_apis_request: crate::models::AddApisRequest
}

/// struct for passing parameters to the method [`delete_api`]
#[derive(Clone, Debug)]
pub struct DeleteApiParams {
    /// The API's id.
    pub api_id: String
}

/// struct for passing parameters to the method [`get_api`]
#[derive(Clone, Debug)]
pub struct GetApiParams {
    /// The API's id.
    pub api_id: String
}

/// struct for passing parameters to the method [`update_api_applications`]
#[derive(Clone, Debug)]
pub struct UpdateApiApplicationsParams {
    /// The identifier for the API.
    pub api_id: String,
    /// The applications you want to connect or disconnect.
    pub update_api_applications_request: crate::models::UpdateApiApplicationsRequest
}


/// struct for typed errors of method [`add_apis`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddApisError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_api`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApiError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_api`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApiError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_apis`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApisError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_api_applications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApiApplicationsError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}


/// Add APIs. 
pub async fn add_apis(configuration: &configuration::Configuration, params: AddApisParams) -> Result<crate::models::SuccessResponse, Error<AddApisError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let add_apis_request = params.add_apis_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/apis", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&add_apis_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddApisError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes API. 
pub async fn delete_api(configuration: &configuration::Configuration, params: DeleteApiParams) -> Result<crate::models::SuccessResponse, Error<DeleteApiError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let api_id = params.api_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/apis/{api_id}", local_var_configuration.base_path, api_id=crate::apis::urlencode(api_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteApiError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the details of the API. 
pub async fn get_api(configuration: &configuration::Configuration, params: GetApiParams) -> Result<crate::models::Api, Error<GetApiError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let api_id = params.api_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/apis/{api_id}", local_var_configuration.base_path, api_id=crate::apis::urlencode(api_id));
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
        let local_var_entity: Option<GetApiError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of APIs. 
pub async fn get_apis(configuration: &configuration::Configuration) -> Result<crate::models::Apis, Error<GetApisError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/apis", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetApisError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the applications under that API. 
pub async fn update_api_applications(configuration: &configuration::Configuration, params: UpdateApiApplicationsParams) -> Result<crate::models::SuccessResponse, Error<UpdateApiApplicationsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let api_id = params.api_id;
    let update_api_applications_request = params.update_api_applications_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/apis/{api_id}/applications", local_var_configuration.base_path, api_id=crate::apis::urlencode(api_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_api_applications_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateApiApplicationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

