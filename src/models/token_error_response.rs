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
pub struct TokenErrorResponse {
    /// Error.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The error description.
    #[serde(rename = "error_description", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

impl TokenErrorResponse {
    pub fn new() -> TokenErrorResponse {
        TokenErrorResponse {
            error: None,
            error_description: None,
        }
    }
}

