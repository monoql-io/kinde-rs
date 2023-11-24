# \SubscribersApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subscriber**](SubscribersApi.md#create_subscriber) | **POST** /api/v1/subscribers | Create Subscriber
[**get_subscriber**](SubscribersApi.md#get_subscriber) | **GET** /api/v1/subscribers/{subscriber_id} | Get Subscriber
[**get_subscribers**](SubscribersApi.md#get_subscribers) | **GET** /api/v1/subscribers | List Subscribers



## create_subscriber

> crate::models::CreateSubscriberSuccessResponse create_subscriber(first_name, last_name, email)
Create Subscriber

Create subscriber.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**first_name** | **String** | Subscriber's first name. | [required] |
**last_name** | Option<**String**> | Subscriber's last name. | [required] |
**email** | Option<**String**> | The email address of the subscriber. | [required] |

### Return type

[**crate::models::CreateSubscriberSuccessResponse**](create_subscriber_success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriber

> crate::models::GetSubscriberResponse get_subscriber(subscriber_id)
Get Subscriber

Retrieve a subscriber record. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscriber_id** | **String** | The subscriber's id. | [required] |

### Return type

[**crate::models::GetSubscriberResponse**](get_subscriber_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscribers

> crate::models::GetSubscribersResponse get_subscribers(sort, page_size, next_token)
List Subscribers

The returned list can be sorted by full name or email address in ascending or descending order. The number of records to return at a time can also be controlled using the `page_size` query string parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Field and order to sort the result by. |  |
**page_size** | Option<**i32**> | Number of results per page. Defaults to 10 if parameter not sent. |  |
**next_token** | Option<**String**> | A string to get the next page of results if there are more results. |  |

### Return type

[**crate::models::GetSubscribersResponse**](get_subscribers_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

