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
pub struct GetOrganizationFeatureFlagsResponseFeatureFlagsValue {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetOrganizationFeatureFlagsResponseFeatureFlagsValue {
    pub fn new() -> GetOrganizationFeatureFlagsResponseFeatureFlagsValue {
        GetOrganizationFeatureFlagsResponseFeatureFlagsValue {
            r#type: None,
            value: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "str")]
    Str,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "bool")]
    Bool,
}

impl Default for Type {
    fn default() -> Type {
        Self::Str
    }
}
