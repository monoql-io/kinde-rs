# \PermissionsApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_permission**](PermissionsApi.md#create_permission) | **POST** /api/v1/permissions | Create Permission
[**delete_permission**](PermissionsApi.md#delete_permission) | **DELETE** /api/v1/permissions/{permission_id} | Delete Permission
[**get_permissions**](PermissionsApi.md#get_permissions) | **GET** /api/v1/permissions | List Permissions
[**update_permissions**](PermissionsApi.md#update_permissions) | **PATCH** /api/v1/permissions/{permission_id} | Update Permission



## create_permission

> crate::models::SuccessResponse create_permission(create_permission_request)
Create Permission

Create a new permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_permission_request** | Option<[**CreatePermissionRequest**](CreatePermissionRequest.md)> | Permission details. |  |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_permission

> crate::models::SuccessResponse delete_permission(permission_id)
Delete Permission

Delete permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_id** | **String** | The identifier for the permission. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permissions

> crate::models::GetPermissionsResponse get_permissions(sort, page_size, next_token)
List Permissions

The returned list can be sorted by permission name or permission ID in ascending or descending order. The number of records to return at a time can also be controlled using the `page_size` query string parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Field and order to sort the result by. |  |
**page_size** | Option<**i32**> | Number of results per page. Defaults to 10 if parameter not sent. |  |
**next_token** | Option<**String**> | A string to get the next page of results if there are more results. |  |

### Return type

[**crate::models::GetPermissionsResponse**](get_permissions_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_permissions

> crate::models::SuccessResponse update_permissions(permission_id, create_permission_request)
Update Permission

Update permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_id** | **i32** | The identifier for the permission. | [required] |
**create_permission_request** | Option<[**CreatePermissionRequest**](CreatePermissionRequest.md)> | Permission details. |  |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

