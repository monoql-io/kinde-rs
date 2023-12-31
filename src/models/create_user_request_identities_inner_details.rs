/*
 * Kinde Management API
 *
 * Provides endpoints to manage your Kinde Businesses
 *
 * The version of the OpenAPI document: 1
 * Contact: support@kinde.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateUserRequestIdentitiesInnerDetails : Additional details required to create the user.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUserRequestIdentitiesInnerDetails {
    /// The email address of the user.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl CreateUserRequestIdentitiesInnerDetails {
    /// Additional details required to create the user.
    pub fn new() -> CreateUserRequestIdentitiesInnerDetails {
        CreateUserRequestIdentitiesInnerDetails {
            email: None,
        }
    }
}


