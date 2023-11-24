# \OrganizationsApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_organization_users**](OrganizationsApi.md#add_organization_users) | **POST** /api/v1/organizations/{org_code}/users | Add Organization Users
[**create_organization**](OrganizationsApi.md#create_organization) | **POST** /api/v1/organization | Create Organization
[**create_organization_user_permission**](OrganizationsApi.md#create_organization_user_permission) | **POST** /api/v1/organizations/{org_code}/users/{user_id}/permissions | Add Organization User Permission
[**create_organization_user_role**](OrganizationsApi.md#create_organization_user_role) | **POST** /api/v1/organizations/{org_code}/users/{user_id}/roles | Add Organization User Role
[**delete_organization**](OrganizationsApi.md#delete_organization) | **DELETE** /api/v1/organization/{org_code} | Delete Organization
[**delete_organization_feature_flag_override**](OrganizationsApi.md#delete_organization_feature_flag_override) | **DELETE** /api/v1/organizations/{org_code}/feature_flags/{feature_flag_key} | Delete Organization Feature Flag Override
[**delete_organization_feature_flag_overrides**](OrganizationsApi.md#delete_organization_feature_flag_overrides) | **DELETE** /api/v1/organizations/{org_code}/feature_flags | Delete Organization Feature Flag Overrides
[**delete_organization_user_permission**](OrganizationsApi.md#delete_organization_user_permission) | **DELETE** /api/v1/organizations/{org_code}/users/{user_id}/permissions/{permission_id} | Delete Organization User Permission
[**delete_organization_user_role**](OrganizationsApi.md#delete_organization_user_role) | **DELETE** /api/v1/organizations/{org_code}/users/{user_id}/roles/{role_id} | Delete Organization User Role
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /api/v1/organization | Get Organization
[**get_organization_feature_flags**](OrganizationsApi.md#get_organization_feature_flags) | **GET** /api/v1/organizations/{org_code}/feature_flags | List Organization Feature Flags
[**get_organization_user_permissions**](OrganizationsApi.md#get_organization_user_permissions) | **GET** /api/v1/organizations/{org_code}/users/{user_id}/permissions | List Organization User Permissions
[**get_organization_user_roles**](OrganizationsApi.md#get_organization_user_roles) | **GET** /api/v1/organizations/{org_code}/users/{user_id}/roles | List Organization User Roles
[**get_organization_users**](OrganizationsApi.md#get_organization_users) | **GET** /api/v1/organizations/{org_code}/users | List Organization Users
[**get_organizations**](OrganizationsApi.md#get_organizations) | **GET** /api/v1/organizations | List Organizations
[**remove_organization_user**](OrganizationsApi.md#remove_organization_user) | **DELETE** /api/v1/organizations/{org_code}/users/{user_id} | Remove Organization User
[**update_organization**](OrganizationsApi.md#update_organization) | **PATCH** /api/v1/organization/{org_code} | Update Organization
[**update_organization_feature_flag_override**](OrganizationsApi.md#update_organization_feature_flag_override) | **PATCH** /api/v1/organizations/{org_code}/feature_flags/{feature_flag_key} | Update Organization Feature Flag Override
[**update_organization_users**](OrganizationsApi.md#update_organization_users) | **PATCH** /api/v1/organizations/{org_code}/users | Update Organization Users



## add_organization_users

> crate::models::AddOrganizationUsersResponse add_organization_users(org_code, add_organization_users_request)
Add Organization Users

Add existing users to an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The organization's code. | [required] |
**add_organization_users_request** | Option<[**AddOrganizationUsersRequest**](AddOrganizationUsersRequest.md)> |  |  |

### Return type

[**crate::models::AddOrganizationUsersResponse**](add_organization_users_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization

> crate::models::CreateOrganizationResponse create_organization(create_organization_request)
Create Organization

Create an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_organization_request** | Option<[**CreateOrganizationRequest**](CreateOrganizationRequest.md)> | Organization details. |  |

### Return type

[**crate::models::CreateOrganizationResponse**](create_organization_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization_user_permission

> crate::models::SuccessResponse create_organization_user_permission(org_code, user_id, create_organization_user_permission_request)
Add Organization User Permission

Add permission to an organization user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The organization's code. | [required] |
**user_id** | **String** | The user's id. | [required] |
**create_organization_user_permission_request** | [**CreateOrganizationUserPermissionRequest**](CreateOrganizationUserPermissionRequest.md) | Permission details. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization_user_role

> crate::models::SuccessResponse create_organization_user_role(org_code, user_id, create_organization_user_role_request)
Add Organization User Role

Add role to an organization user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The organization's code. | [required] |
**user_id** | **String** | The user's id. | [required] |
**create_organization_user_role_request** | [**CreateOrganizationUserRoleRequest**](CreateOrganizationUserRoleRequest.md) | Role details. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> delete_organization(org_code)
Delete Organization

Delete an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The identifier for the organization. | [required] |

### Return type

 (empty response body)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_feature_flag_override

> crate::models::SuccessResponse delete_organization_feature_flag_override(org_code, feature_flag_key)
Delete Organization Feature Flag Override

Delete organization feature flag override.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The identifier for the organization. | [required] |
**feature_flag_key** | **String** | The identifier for the feature flag. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_feature_flag_overrides

> crate::models::SuccessResponse delete_organization_feature_flag_overrides(org_code)
Delete Organization Feature Flag Overrides

Delete all organization feature flag overrides.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The identifier for the organization. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_user_permission

> crate::models::SuccessResponse delete_organization_user_permission(org_code, user_id, permission_id)
Delete Organization User Permission

Delete permission for an organization user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The organization's code. | [required] |
**user_id** | **String** | The user's id. | [required] |
**permission_id** | **String** | The permission id. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_user_role

> crate::models::SuccessResponse delete_organization_user_role(org_code, user_id, role_id)
Delete Organization User Role

Delete role for an organization user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The organization's code. | [required] |
**user_id** | **String** | The user's id. | [required] |
**role_id** | **String** | The role id. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> crate::models::Organization get_organization(code)
Get Organization

Gets an organization given the organization's code. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | Option<**String**> | The organization's code. |  |

### Return type

[**crate::models::Organization**](organization.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_feature_flags

> crate::models::GetOrganizationFeatureFlagsResponse get_organization_feature_flags(org_code)
List Organization Feature Flags

Get all organization feature flags.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The identifier for the organization. | [required] |

### Return type

[**crate::models::GetOrganizationFeatureFlagsResponse**](get_organization_feature_flags_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_user_permissions

> crate::models::GetOrganizationsUserPermissionsResponse get_organization_user_permissions(org_code, user_id, expand)
List Organization User Permissions

Get permissions for an organization user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The organization's code. | [required] |
**user_id** | **String** | The user's id. | [required] |
**expand** | Option<**String**> | Specify additional data to retrieve. Use \"roles\". |  |

### Return type

[**crate::models::GetOrganizationsUserPermissionsResponse**](get_organizations_user_permissions_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_user_roles

> crate::models::GetOrganizationsUserRolesResponse get_organization_user_roles(org_code, user_id)
List Organization User Roles

Get roles for an organization user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The organization's code. | [required] |
**user_id** | **String** | The user's id. | [required] |

### Return type

[**crate::models::GetOrganizationsUserRolesResponse**](get_organizations_user_roles_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_users

> crate::models::GetOrganizationUsersResponse get_organization_users(org_code, sort, page_size, next_token, permissions, roles)
List Organization Users

Get users in an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The organization's code. | [required] |
**sort** | Option<**String**> | Field and order to sort the result by. |  |
**page_size** | Option<**i32**> | Number of results per page. Defaults to 10 if parameter not sent. |  |
**next_token** | Option<**String**> | A string to get the next page of results if there are more results. |  |
**permissions** | Option<**String**> | Filter by user permissions comma separated (where all match) |  |
**roles** | Option<**String**> | Filter by user roles comma separated (where all match) |  |

### Return type

[**crate::models::GetOrganizationUsersResponse**](get_organization_users_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations

> crate::models::GetOrganizationsResponse get_organizations(sort, page_size, next_token)
List Organizations

Get a list of organizations. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Field and order to sort the result by. |  |
**page_size** | Option<**i32**> | Number of results per page. Defaults to 10 if parameter not sent. |  |
**next_token** | Option<**String**> | A string to get the next page of results if there are more results. |  |

### Return type

[**crate::models::GetOrganizationsResponse**](get_organizations_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_organization_user

> crate::models::SuccessResponse remove_organization_user(org_code, user_id)
Remove Organization User

Remove user from an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The organization's code. | [required] |
**user_id** | **String** | The user's id. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> crate::models::SuccessResponse update_organization(org_code, update_organization_request)
Update Organization

Update an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The identifier for the organization. | [required] |
**update_organization_request** | Option<[**UpdateOrganizationRequest**](UpdateOrganizationRequest.md)> | Organization details. |  |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_feature_flag_override

> crate::models::SuccessResponse update_organization_feature_flag_override(org_code, feature_flag_key, value)
Update Organization Feature Flag Override

Update organization feature flag override.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The identifier for the organization | [required] |
**feature_flag_key** | **String** | The identifier for the feature flag | [required] |
**value** | **String** | Override value | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_users

> crate::models::UpdateOrganizationUsersResponse update_organization_users(org_code, update_organization_users_request)
Update Organization Users

Update users that belong to an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_code** | **String** | The organization's code. | [required] |
**update_organization_users_request** | Option<[**UpdateOrganizationUsersRequest**](UpdateOrganizationUsersRequest.md)> |  |  |

### Return type

[**crate::models::UpdateOrganizationUsersResponse**](update_organization_users_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

