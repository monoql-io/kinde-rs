# \IndustriesApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_industries**](IndustriesApi.md#get_industries) | **GET** /api/v1/industries | List industries and industry keys.



## get_industries

> crate::models::SuccessResponse get_industries(industry_key, name)
List industries and industry keys.

Get a list of industries and associated industry keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**industry_key** | Option<**String**> | Industry Key. |  |
**name** | Option<**String**> | Industry name. |  |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

