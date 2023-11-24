/*
 * Kinde Management API
 *
 * Provides endpoints to manage your Kinde Businesses
 *
 * The version of the OpenAPI document: 1
 * Contact: support@kinde.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPermissionsResponse {
    /// Response code.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Response message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<crate::models::Permissions>>,
    /// Pagination token.
    #[serde(rename = "next_token", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl GetPermissionsResponse {
    pub fn new() -> GetPermissionsResponse {
        GetPermissionsResponse {
            code: None,
            message: None,
            permissions: None,
            next_token: None,
        }
    }
}

