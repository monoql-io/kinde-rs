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
pub struct UserProfile {
    /// Unique id of the user in Kinde.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Default email address of the user in Kinde.
    #[serde(rename = "preferred_email", skip_serializing_if = "Option::is_none")]
    pub preferred_email: Option<String>,
    /// Value of the user's id in a third-party system when the user is imported into Kinde.
    #[serde(rename = "provided_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub provided_id: Option<Option<String>>,
    /// User's last name.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// User's first name.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// URL that point's to the user's picture or avatar
    #[serde(rename = "picture", skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
}

impl UserProfile {
    pub fn new() -> UserProfile {
        UserProfile {
            id: None,
            preferred_email: None,
            provided_id: None,
            last_name: None,
            first_name: None,
            picture: None,
        }
    }
}

