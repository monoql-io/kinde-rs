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
pub struct ReplaceRedirectCallbackUrlsRequest {
    /// Array of callback urls.
    #[serde(rename = "urls", skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

impl ReplaceRedirectCallbackUrlsRequest {
    pub fn new() -> ReplaceRedirectCallbackUrlsRequest {
        ReplaceRedirectCallbackUrlsRequest {
            urls: None,
        }
    }
}


