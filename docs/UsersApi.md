# \UsersApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **POST** /api/v1/user | Create User
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /api/v1/user | Delete User
[**get_user_data**](UsersApi.md#get_user_data) | **GET** /api/v1/user | Get User
[**get_users**](UsersApi.md#get_users) | **GET** /api/v1/users | List Users
[**refresh_user_claims**](UsersApi.md#refresh_user_claims) | **POST** /api/v1/users/{user_id}/refresh_claims | Refresh User Claims and Invalidate Cache
[**update_user**](UsersApi.md#update_user) | **PATCH** /api/v1/user | Update User
[**update_user_feature_flag_override**](UsersApi.md#update_user_feature_flag_override) | **PATCH** /api/v1/users/{user_id}/feature_flags/{feature_flag_key} | Update User Feature Flag Override



## create_user

> crate::models::CreateUserResponse create_user(create_user_request)
Create User

Creates a user record and optionally zero or more identities for the user. An example identity could be the email address of the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | Option<[**CreateUserRequest**](CreateUserRequest.md)> | The details of the user to create. |  |

### Return type

[**crate::models::CreateUserResponse**](create_user_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> crate::models::SuccessResponse delete_user(id, is_delete_profile)
Delete User

Delete a user record. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user's id. | [required] |
**is_delete_profile** | Option<**bool**> | Delete all data and remove the user's profile from all of Kinde, including the subscriber list |  |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_data

> crate::models::User get_user_data(id, expand)
Get User

Retrieve a user record. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user's id. | [required] |
**expand** | Option<**String**> | Specify additional data to retrieve. Use \"organizations\" and/or \"identities\". |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> crate::models::UsersResponse get_users(sort, page_size, user_id, next_token, email, expand)
List Users

The returned list can be sorted by full name or email address in ascending or descending order. The number of records to return at a time can also be controlled using the `page_size` query string parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Field and order to sort the result by. |  |
**page_size** | Option<**i32**> | Number of results per page. Defaults to 10 if parameter not sent. |  |
**user_id** | Option<**String**> | ID of the user to filter by. |  |
**next_token** | Option<**String**> | A string to get the next page of results if there are more results. |  |
**email** | Option<**String**> | Filter the results by email address. The query string should be comma separated and url encoded. |  |
**expand** | Option<**String**> | Specify additional data to retrieve. Use \"organizations\" and/or \"identities\". |  |

### Return type

[**crate::models::UsersResponse**](users_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_user_claims

> crate::models::SuccessResponse refresh_user_claims(user_id)
Refresh User Claims and Invalidate Cache

Refreshes the user's claims and invalidates the current cache. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The id of the user whose claims needs to be updated. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::UpdateUserResponse update_user(update_user_request, id)
Update User

Update a user record. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) | The user to update. | [required] |
**id** | Option<**String**> | The user's id. |  |

### Return type

[**crate::models::UpdateUserResponse**](update_user_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_feature_flag_override

> crate::models::SuccessResponse update_user_feature_flag_override(user_id, feature_flag_key, value)
Update User Feature Flag Override

Update user feature flag override.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The identifier for the user | [required] |
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

