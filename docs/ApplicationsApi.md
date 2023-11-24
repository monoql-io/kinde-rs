# \ApplicationsApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_application**](ApplicationsApi.md#create_application) | **POST** /api/v1/applications | Create Application
[**delete_application**](ApplicationsApi.md#delete_application) | **DELETE** /api/v1/applications/{application_id} | Delete Application
[**get_application**](ApplicationsApi.md#get_application) | **GET** /api/v1/applications/{application_id} | Get Application
[**get_applications**](ApplicationsApi.md#get_applications) | **GET** /api/v1/applications | List Applications
[**update_application**](ApplicationsApi.md#update_application) | **PATCH** /api/v1/applications/{application_id} | Update Application



## create_application

> crate::models::CreateApplicationResponse create_application(create_application_request)
Create Application

Create an application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_application_request** | Option<[**CreateApplicationRequest**](CreateApplicationRequest.md)> | Application details. |  |

### Return type

[**crate::models::CreateApplicationResponse**](create_application_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application

> crate::models::SuccessResponse delete_application(application_id)
Delete Application

Delete application. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The identifier for the application. | [required] |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application

> crate::models::GetApplicationResponse get_application(application_id)
Get Application

Gets an application given the application's id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The identifier for the application. | [required] |

### Return type

[**crate::models::GetApplicationResponse**](get_application_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_applications

> crate::models::GetApplicationsResponse get_applications(sort, page_size, next_token)
List Applications

Get a list of applications. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Field and order to sort the result by. |  |
**page_size** | Option<**i32**> | Number of results per page. Defaults to 10 if parameter not sent. |  |
**next_token** | Option<**String**> | A string to get the next page of results if there are more results. |  |

### Return type

[**crate::models::GetApplicationsResponse**](get_applications_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application

> update_application(application_id, update_application_request)
Update Application

Update an application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The identifier for the application. | [required] |
**update_application_request** | Option<[**UpdateApplicationRequest**](UpdateApplicationRequest.md)> | Application details. |  |

### Return type

 (empty response body)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

