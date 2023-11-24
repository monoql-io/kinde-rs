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
pub struct UpdateOrganizationUsersRequest {
    /// Users to add, update or remove from the organization.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::UpdateOrganizationUsersRequestUsersInner>>,
}

impl UpdateOrganizationUsersRequest {
    pub fn new() -> UpdateOrganizationUsersRequest {
        UpdateOrganizationUsersRequest {
            users: None,
        }
    }
}

