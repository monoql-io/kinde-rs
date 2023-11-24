# \EnvironmentsApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_environement_feature_flag_override**](EnvironmentsApi.md#delete_environement_feature_flag_override) | **DELETE** /api/v1/environment/feature_flags/{feature_flag_key} | Delete Environment Feature Flag Override
[**delete_environement_feature_flag_overrides**](EnvironmentsApi.md#delete_environement_feature_flag_overrides) | **DELETE** /api/v1/environment/feature_flags | Delete Environment Feature Flag Overrides
[**get_environement_feature_flags**](EnvironmentsApi.md#get_environement_feature_flags) | **GET** /api/v1/environment/feature_flags | List Environment Feature Flags
[**update_environement_feature_flag_override**](EnvironmentsApi.md#update_environement_feature_flag_override) | **PATCH** /api/v1/environment/feature_flags/{feature_flag_key} | Update Environment Feature Flag Override



## delete_environement_feature_flag_override

> crate::models::SuccessResponse delete_environement_feature_flag_override(feature_flag_key)
Delete Environment Feature Flag Override

Delete environment feature flag override.

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


## delete_environement_feature_flag_overrides

> crate::models::SuccessResponse delete_environement_feature_flag_overrides()
Delete Environment Feature Flag Overrides

Delete all environment feature flag overrides.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environement_feature_flags

> crate::models::GetEnvironmentFeatureFlagsResponse get_environement_feature_flags()
List Environment Feature Flags

Get environment feature flags.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetEnvironmentFeatureFlagsResponse**](get_environment_feature_flags_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_environement_feature_flag_override

> crate::models::SuccessResponse update_environement_feature_flag_override(feature_flag_key, update_environement_feature_flag_override_request)
Update Environment Feature Flag Override

Update environment feature flag override.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_flag_key** | **String** | The identifier for the feature flag. | [required] |
**update_environement_feature_flag_override_request** | [**UpdateEnvironementFeatureFlagOverrideRequest**](UpdateEnvironementFeatureFlagOverrideRequest.md) | Flag details. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

