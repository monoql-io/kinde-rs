# \ConnectedAppsApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_connected_app_auth_url**](ConnectedAppsApi.md#get_connected_app_auth_url) | **GET** /api/v1/connected_apps/auth_url | Get Connected App URL
[**get_connected_app_token**](ConnectedAppsApi.md#get_connected_app_token) | **GET** /api/v1/connected_apps/token | Get Connected App Token
[**revoke_connected_app_token**](ConnectedAppsApi.md#revoke_connected_app_token) | **POST** /api/v1/connected_apps/revoke | Revoke Connected App Token



## get_connected_app_auth_url

> crate::models::ConnectedAppsAuthUrl get_connected_app_auth_url(key_code_ref, user_id, org_code)
Get Connected App URL

Get a URL that authenticates and authorizes a user to a third-party connected app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_code_ref** | **String** | The unique key code reference of the connected app to authenticate against. | [required] |
**user_id** | Option<**String**> | The id of the user that needs to authenticate to the third-party connected app. |  |
**org_code** | Option<**String**> | The code of the Kinde organization that needs to authenticate to the third-party connected app. |  |

### Return type

[**crate::models::ConnectedAppsAuthUrl**](connected_apps_auth_url.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connected_app_token

> crate::models::ConnectedAppsAccessToken get_connected_app_token(session_id)
Get Connected App Token

Get an access token that can be used to call the third-party provider linked to the connected app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The unique sesssion id reprensenting the login session of a user. | [required] |

### Return type

[**crate::models::ConnectedAppsAccessToken**](connected_apps_access_token.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_connected_app_token

> crate::models::SuccessResponse revoke_connected_app_token(session_id)
Revoke Connected App Token

Revoke the tokens linked to the connected app session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The unique sesssion id reprensenting the login session of a user. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

