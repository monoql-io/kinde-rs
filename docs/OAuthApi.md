# \OAuthApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user**](OAuthApi.md#get_user) | **GET** /oauth2/user_profile | Get User Profile
[**get_user_profile_v2**](OAuthApi.md#get_user_profile_v2) | **GET** /oauth2/v2/user_profile | Returns the details of the currently logged in user
[**token_introspection**](OAuthApi.md#token_introspection) | **POST** /oauth2/introspect | Get token details
[**token_revocation**](OAuthApi.md#token_revocation) | **POST** /oauth2/revoke | Revoke token



## get_user

> crate::models::UserProfile get_user()
Get User Profile

Contains the id, names and email of the currently logged in user. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserProfile**](user_profile.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_profile_v2

> crate::models::UserProfileV2 get_user_profile_v2()
Returns the details of the currently logged in user

Contains the id, names, profile picture URL and email of the currently logged in user. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserProfileV2**](user_profile_v2.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_introspection

> crate::models::TokenIntrospect token_introspection(token, token_type)
Get token details

Retrieve information about the provided token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | The token to be introspected. |  |
**token_type** | Option<**String**> | The provided token's type. |  |

### Return type

[**crate::models::TokenIntrospect**](token_introspect.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_revocation

> token_revocation(token, client_id, client_secret)
Revoke token

Revoke a previously issued token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | The token to be revoked. |  |
**client_id** | Option<**String**> | The identifier for your client. |  |
**client_secret** | Option<**String**> | The secret associated with your client. |  |

### Return type

 (empty response body)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

