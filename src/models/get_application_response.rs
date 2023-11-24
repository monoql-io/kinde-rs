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
pub struct GetApplicationResponse {
    /// Response code.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Response message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<crate::models::GetApplicationResponseApplication>>,
}

impl GetApplicationResponse {
    pub fn new() -> GetApplicationResponse {
        GetApplicationResponse {
            code: None,
            message: None,
            application: None,
        }
    }
}

