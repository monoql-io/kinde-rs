# \CallbacksApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_logout_redirect_urls**](CallbacksApi.md#add_logout_redirect_urls) | **POST** /api/v1/applications/{app_id}/auth_logout_urls | Add Logout Redirect URLs
[**add_redirect_callback_urls**](CallbacksApi.md#add_redirect_callback_urls) | **POST** /api/v1/applications/{app_id}/auth_redirect_urls | Add Redirect Callback URLs
[**delete_callback_urls**](CallbacksApi.md#delete_callback_urls) | **DELETE** /api/v1/applications/{app_id}/auth_redirect_urls | Delete Callback URLs
[**delete_logout_urls**](CallbacksApi.md#delete_logout_urls) | **DELETE** /api/v1/applications/{app_id}/auth_logout_urls | Delete Logout URLs
[**get_callback_urls**](CallbacksApi.md#get_callback_urls) | **GET** /api/v1/applications/{app_id}/auth_redirect_urls | List Callback URLs
[**get_logout_urls**](CallbacksApi.md#get_logout_urls) | **GET** /api/v1/applications/{app_id}/auth_logout_urls | List Logout URLs
[**replace_logout_redirect_urls**](CallbacksApi.md#replace_logout_redirect_urls) | **PUT** /api/v1/applications/{app_id}/auth_logout_urls | Replace Logout Redirect URLs
[**replace_redirect_callback_urls**](CallbacksApi.md#replace_redirect_callback_urls) | **PUT** /api/v1/applications/{app_id}/auth_redirect_urls | Replace Redirect Callback URLs



## add_logout_redirect_urls

> crate::models::SuccessResponse add_logout_redirect_urls(app_id, replace_logout_redirect_urls_request)
Add Logout Redirect URLs

Add additional logout redirect URLs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The identifier for the application. | [required] |
**replace_logout_redirect_urls_request** | [**ReplaceLogoutRedirectUrlsRequest**](ReplaceLogoutRedirectUrlsRequest.md) | Callback details. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_redirect_callback_urls

> crate::models::SuccessResponse add_redirect_callback_urls(app_id, replace_redirect_callback_urls_request)
Add Redirect Callback URLs

Add additional redirect callback URLs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The identifier for the application. | [required] |
**replace_redirect_callback_urls_request** | [**ReplaceRedirectCallbackUrlsRequest**](ReplaceRedirectCallbackUrlsRequest.md) | Callback details. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_callback_urls

> crate::models::SuccessResponse delete_callback_urls(app_id, urls)
Delete Callback URLs

Delete callback URLs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The identifier for the application. | [required] |
**urls** | **String** | Urls to delete, comma separated and url encoded. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_logout_urls

> crate::models::SuccessResponse delete_logout_urls(app_id, urls)
Delete Logout URLs

Delete logout URLs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The identifier for the application. | [required] |
**urls** | **String** | Urls to delete, comma separated and url encoded. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_callback_urls

> crate::models::RedirectCallbackUrls get_callback_urls(app_id)
List Callback URLs

Returns an application's redirect callback URLs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The identifier for the application. | [required] |

### Return type

[**crate::models::RedirectCallbackUrls**](redirect_callback_urls.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logout_urls

> crate::models::LogoutRedirectUrls get_logout_urls(app_id)
List Logout URLs

Returns an application's logout redirect URLs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The identifier for the application. | [required] |

### Return type

[**crate::models::LogoutRedirectUrls**](logout_redirect_urls.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_logout_redirect_urls

> crate::models::SuccessResponse replace_logout_redirect_urls(app_id, replace_logout_redirect_urls_request)
Replace Logout Redirect URLs

Replace all logout redirect URLs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The identifier for the application. | [required] |
**replace_logout_redirect_urls_request** | [**ReplaceLogoutRedirectUrlsRequest**](ReplaceLogoutRedirectUrlsRequest.md) | Callback details. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_redirect_callback_urls

> crate::models::SuccessResponse replace_redirect_callback_urls(app_id, replace_redirect_callback_urls_request)
Replace Redirect Callback URLs

Replace all redirect callback URLs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The identifier for the application. | [required] |
**replace_redirect_callback_urls_request** | [**ReplaceRedirectCallbackUrlsRequest**](ReplaceRedirectCallbackUrlsRequest.md) | Callback details. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

