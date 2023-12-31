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
pub struct CreateSubscriberSuccessResponseSubscriber {
    /// A unique identifier for the subscriber.
    #[serde(rename = "subscriber_id", skip_serializing_if = "Option::is_none")]
    pub subscriber_id: Option<String>,
}

impl CreateSubscriberSuccessResponseSubscriber {
    pub fn new() -> CreateSubscriberSuccessResponseSubscriber {
        CreateSubscriberSuccessResponseSubscriber {
            subscriber_id: None,
        }
    }
}


