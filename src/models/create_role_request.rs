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
pub struct CreateRoleRequest {
    /// The role's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The role's description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The role identifier to use in code.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Set role as default for new users.
    #[serde(rename = "is_default_role", skip_serializing_if = "Option::is_none")]
    pub is_default_role: Option<bool>,
}

impl CreateRoleRequest {
    pub fn new() -> CreateRoleRequest {
        CreateRoleRequest {
            name: None,
            description: None,
            key: None,
            is_default_role: None,
        }
    }
}

