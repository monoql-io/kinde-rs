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
pub struct Apis {
    /// Unique id of the API.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The API's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The logical identifier for the API.
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    /// Whether it is the management API or not.
    #[serde(rename = "is_management_api", skip_serializing_if = "Option::is_none")]
    pub is_management_api: Option<bool>,
}

impl Apis {
    pub fn new() -> Apis {
        Apis {
            id: None,
            name: None,
            audience: None,
            is_management_api: None,
        }
    }
}


