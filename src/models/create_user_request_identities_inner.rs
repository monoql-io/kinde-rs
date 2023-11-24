/*
 * Kinde Management API
 *
 * Provides endpoints to manage your Kinde Businesses
 *
 * The version of the OpenAPI document: 1
 * Contact: support@kinde.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateUserRequestIdentitiesInner : The result of the user creation operation.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUserRequestIdentitiesInner {
    /// The type of identity to create, for e.g. email.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Box<crate::models::CreateUserRequestIdentitiesInnerDetails>>,
}

impl CreateUserRequestIdentitiesInner {
    /// The result of the user creation operation.
    pub fn new() -> CreateUserRequestIdentitiesInner {
        CreateUserRequestIdentitiesInner {
            r#type: None,
            details: None,
        }
    }
}

/// The type of identity to create, for e.g. email.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "email")]
    Email,
}

impl Default for Type {
    fn default() -> Type {
        Self::Email
    }
}
