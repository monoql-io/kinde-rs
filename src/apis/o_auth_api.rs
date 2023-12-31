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

/// struct for passing parameters to the method [`token_introspection`]
#[derive(Clone, Debug)]
pub struct TokenIntrospectionParams {
    /// The token to be introspected.
    pub token: Option<String>,
    /// The provided token's type.
    pub token_type: Option<String>
}

/// struct for passing parameters to the method [`token_revocation`]
#[derive(Clone, Debug)]
pub struct TokenRevocationParams {
    /// The token to be revoked.
    pub token: Option<String>,
    /// The identifier for your client.
    pub client_id: Option<String>,
    /// The secret associated with your client.
    pub client_secret: Option<String>
}


/// struct for typed errors of method [`get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserError {
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_profile_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserProfileV2Error {
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`token_introspection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenIntrospectionError {
    Status401(crate::models::TokenErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`token_revocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenRevocationError {
    Status401(crate::models::TokenErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}


/// Contains the id, names and email of the currently logged in user. 
pub async fn get_user(configuration: &configuration::Configuration) -> Result<crate::models::UserProfile, Error<GetUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth2/user_profile", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Contains the id, names, profile picture URL and email of the currently logged in user. 
pub async fn get_user_profile_v2(configuration: &configuration::Configuration) -> Result<crate::models::UserProfileV2, Error<GetUserProfileV2Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth2/v2/user_profile", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetUserProfileV2Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve information about the provided token.
pub async fn token_introspection(configuration: &configuration::Configuration, params: TokenIntrospectionParams) -> Result<crate::models::TokenIntrospect, Error<TokenIntrospectionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let token = params.token;
    let token_type = params.token_type;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth2/introspect", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = token {
        local_var_form_params.insert("token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = token_type {
        local_var_form_params.insert("token_type", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TokenIntrospectionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Revoke a previously issued token.
pub async fn token_revocation(configuration: &configuration::Configuration, params: TokenRevocationParams) -> Result<(), Error<TokenRevocationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let token = params.token;
    let client_id = params.client_id;
    let client_secret = params.client_secret;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/oauth2/revoke", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = token {
        local_var_form_params.insert("token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = client_id {
        local_var_form_params.insert("client_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = client_secret {
        local_var_form_params.insert("client_secret", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<TokenRevocationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

