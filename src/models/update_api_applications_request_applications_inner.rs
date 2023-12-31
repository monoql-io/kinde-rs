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
pub struct UpdateApiApplicationsRequestApplicationsInner {
    /// The application's id.
    #[serde(rename = "id")]
    pub id: String,
    /// Optional operation, set to 'delete' to remove the user from the organization.
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
}

impl UpdateApiApplicationsRequestApplicationsInner {
    pub fn new(id: String) -> UpdateApiApplicationsRequestApplicationsInner {
        UpdateApiApplicationsRequestApplicationsInner {
            id,
            operation: None,
        }
    }
}


