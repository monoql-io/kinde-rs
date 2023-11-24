# \ApisApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_apis**](ApisApi.md#add_apis) | **POST** /api/v1/apis | Add APIs
[**delete_api**](ApisApi.md#delete_api) | **DELETE** /api/v1/apis/{api_id} | Delete API
[**get_api**](ApisApi.md#get_api) | **GET** /api/v1/apis/{api_id} | List API details
[**get_apis**](ApisApi.md#get_apis) | **GET** /api/v1/apis | List APIs
[**update_api_applications**](ApisApi.md#update_api_applications) | **PATCH** /api/v1/apis/{api_id}/applications | Update API Applications



## add_apis

> crate::models::SuccessResponse add_apis(add_apis_request)
Add APIs

Add APIs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_apis_request** | [**AddApisRequest**](AddApisRequest.md) | API details. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api

> crate::models::SuccessResponse delete_api(api_id)
Delete API

Deletes API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_id** | **String** | The API's id. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api

> crate::models::Api get_api(api_id)
List API details

Returns the details of the API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_id** | **String** | The API's id. | [required] |

### Return type

[**crate::models::Api**](api.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_apis

> crate::models::Apis get_apis()
List APIs

Returns a list of APIs. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Apis**](apis.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_applications

> crate::models::SuccessResponse update_api_applications(api_id, update_api_applications_request)
Update API Applications

Update the applications under that API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_id** | **String** | The identifier for the API. | [required] |
**update_api_applications_request** | [**UpdateApiApplicationsRequest**](UpdateApiApplicationsRequest.md) | The applications you want to connect or disconnect. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

