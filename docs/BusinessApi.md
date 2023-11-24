# \BusinessApi

All URIs are relative to *https://app.kinde.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_business**](BusinessApi.md#get_business) | **GET** /api/v1/business | List business details
[**update_business**](BusinessApi.md#update_business) | **PATCH** /api/v1/business | Update business details



## get_business

> crate::models::SuccessResponse get_business(code, name, email, phone, industry, timezone, privacy_url, terms_url)
List business details

Get your business details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Business code. | [required] |
**name** | **String** | Business name. | [required] |
**email** | **String** | Email associated with business. | [required] |
**phone** | Option<**String**> | Phone number associated with business. |  |
**industry** | Option<**String**> | The industry your business is in. |  |
**timezone** | Option<**String**> | The timezone your business is in. |  |
**privacy_url** | Option<**String**> | Your Privacy policy URL. |  |
**terms_url** | Option<**String**> | Your Terms and Conditions URL. |  |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_business

> crate::models::SuccessResponse update_business(business_name, primary_email, primary_phone, industry_key, timezone_id, privacy_url, terms_url, is_show_kinde_branding, is_click_wrap, partner_code)
Update business details

Update business details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_name** | **String** | Business name. | [required] |
**primary_email** | **String** | Email associated with business. | [required] |
**primary_phone** | Option<**String**> | Phone number associated with business. |  |
**industry_key** | Option<**String**> | The key of the industry your business is in. |  |
**timezone_id** | Option<**String**> | The ID of the timezone your business is in. |  |
**privacy_url** | Option<**String**> | Your Privacy policy URL. |  |
**terms_url** | Option<**String**> | Your Terms and Conditions URL. |  |
**is_show_kinde_branding** | Option<**String**> | Display \"Powered by Kinde\" on your sign up, sign in, and subscription pages. |  |
**is_click_wrap** | Option<**bool**> | Show a policy acceptance checkbox on sign up. |  |
**partner_code** | Option<**String**> | Your Kinde Perk code. |  |

### Return type

[**crate::models::SuccessResponse**](success_response.md)

### Authorization

[kindeBearerAuth](../README.md#kindeBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

