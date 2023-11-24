# \FeatureFlagsApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_feature_flag**](FeatureFlagsApi.md#create_feature_flag) | **POST** /api/v1/feature_flags | Create Feature Flag
[**delete_feature_flag**](FeatureFlagsApi.md#delete_feature_flag) | **DELETE** /api/v1/feature_flags/{feature_flag_key} | Delete Feature Flag
[**update_feature_flag**](FeatureFlagsApi.md#update_feature_flag) | **PUT** /api/v1/feature_flags/{feature_flag_key} | Replace Feature Flag



## create_feature_flag

> crate::models::SuccessResponse create_feature_flag(create_feature_flag_request)
Create Feature Flag

Create feature flag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_feature_flag_request** | [**CreateFeatureFlagRequest**](CreateFeatureFlagRequest.md) | Flag details. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feature_flag

> crate::models::SuccessResponse delete_feature_flag(feature_flag_key)
Delete Feature Flag

Delete feature flag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_flag_key** | **String** | The identifier for the feature flag. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feature_flag

> crate::models::SuccessResponse update_feature_flag(feature_flag_key, name, description, r#type, allow_override_level, default_value)
Replace Feature Flag

Update feature flag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_flag_key** | **String** | The key identifier for the feature flag. | [required] |
**name** | **String** | The name of the flag. | [required] |
**description** | **String** | Description of the flag purpose. | [required] |
**r#type** | **String** | The variable type | [required] |
**allow_override_level** | **String** | Allow the flag to be overridden at a different level. | [required] |
**default_value** | **String** | Default value for the flag used by environments and organizations. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

