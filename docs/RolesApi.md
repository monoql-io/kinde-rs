# \RolesApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_role**](RolesApi.md#create_role) | **POST** /api/v1/role | Create Role
[**delete_role**](RolesApi.md#delete_role) | **DELETE** /api/v1/roles/{role_id} | Delete Role
[**get_role_permission**](RolesApi.md#get_role_permission) | **GET** /api/v1/roles/{role_id}/permissions | Get Role Permissions
[**get_roles**](RolesApi.md#get_roles) | **GET** /api/v1/roles | List Roles
[**remove_role_permission**](RolesApi.md#remove_role_permission) | **DELETE** /api/v1/roles/{role_id}/permissions/{permission_id} | Remove Role Permission
[**update_role_permissions**](RolesApi.md#update_role_permissions) | **PATCH** /api/v1/roles/{role_id}/permissions | Update Role Permissions
[**update_roles**](RolesApi.md#update_roles) | **PATCH** /api/v1/roles/{role_id} | Update Role



## create_role

> crate::models::SuccessResponse create_role(create_role_request)
Create Role

Create role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_role_request** | Option<[**CreateRoleRequest**](CreateRoleRequest.md)> | Role details. |  |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> crate::models::SuccessResponse delete_role(role_id)
Delete Role

Delete role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The identifier for the role. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_permission

> Vec<crate::models::RolesPermissionResponseInner> get_role_permission(role_id, sort, page_size, next_token)
Get Role Permissions

Get permissions for a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The role's public id. | [required] |
**sort** | Option<**String**> | Field and order to sort the result by. |  |
**page_size** | Option<**i32**> | Number of results per page. Defaults to 10 if parameter not sent. |  |
**next_token** | Option<**String**> | A string to get the next page of results if there are more results. |  |

### Return type

[**Vec<crate::models::RolesPermissionResponseInner>**](roles_permission_response_inner.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roles

> crate::models::GetRolesResponse get_roles(sort, page_size, next_token)
List Roles

The returned list can be sorted by role name or role ID in ascending or descending order. The number of records to return at a time can also be controlled using the `page_size` query string parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Field and order to sort the result by. |  |
**page_size** | Option<**i32**> | Number of results per page. Defaults to 10 if parameter not sent. |  |
**next_token** | Option<**String**> | A string to get the next page of results if there are more results. |  |

### Return type

[**crate::models::GetRolesResponse**](get_roles_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_role_permission

> crate::models::SuccessResponse remove_role_permission(role_id, permission_id)
Remove Role Permission

Remove a permission from a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The role's public id. | [required] |
**permission_id** | **String** | The permission's public id. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role_permissions

> crate::models::UpdateRolePermissionsResponse update_role_permissions(role_id, update_role_permissions_request)
Update Role Permissions

Update role permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The identifier for the role. | [required] |
**update_role_permissions_request** | [**UpdateRolePermissionsRequest**](UpdateRolePermissionsRequest.md) |  | [required] |

### Return type

[**crate::models::UpdateRolePermissionsResponse**](update_role_permissions_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_roles

> crate::models::SuccessResponse update_roles(role_id, update_roles_request)
Update Role

Update a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The identifier for the role. | [required] |
**update_roles_request** | Option<[**UpdateRolesRequest**](UpdateRolesRequest.md)> | Role details. |  |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

