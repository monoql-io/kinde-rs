# \TimezonesApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_timezones**](TimezonesApi.md#get_timezones) | **GET** /api/v1/timezones | List timezones and timezone IDs.



## get_timezones

> crate::models::SuccessResponse get_timezones(timezone_key, name)
List timezones and timezone IDs.

Get a list of timezones and associated timezone keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timezone_key** | Option<**String**> | Timezone Key. |  |
**name** | Option<**String**> | Timezone. |  |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

