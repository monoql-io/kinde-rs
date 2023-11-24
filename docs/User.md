# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique id of the user in Kinde. | [optional]
**provided_id** | Option<**String**> | External id for user. | [optional]
**preferred_email** | Option<**String**> | Default email address of the user in Kinde. | [optional]
**last_name** | Option<**String**> | User's last name. | [optional]
**first_name** | Option<**String**> | User's first name. | [optional]
**is_suspended** | Option<**bool**> | Whether the user is currently suspended or not. | [optional]
**picture** | Option<**String**> | User's profile picture URL. | [optional]
**total_sign_ins** | Option<**i32**> | Total number of user sign ins. | [optional]
**failed_sign_ins** | Option<**i32**> | Number of consecutive failed user sign ins. | [optional]
**last_signed_in** | Option<**String**> | Last sign in date in ISO 8601 format. | [optional]
**created_on** | Option<**String**> | Date of user creation in ISO 8601 format. | [optional]
**organizations** | Option<**Vec<String>**> | Array of organizations a user belongs to. | [optional]
**identities** | Option<[**Vec<crate::models::UserIdentitiesInner>**](user_identities_inner.md)> | Array of identities belonging to the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


