# \DefaultApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_financial_event_groups**](DefaultApi.md#list_financial_event_groups) | **GET** /finances/v0/financialEventGroups | 
[**list_financial_events**](DefaultApi.md#list_financial_events) | **GET** /finances/v0/financialEvents | 
[**list_financial_events_by_group_id**](DefaultApi.md#list_financial_events_by_group_id) | **GET** /finances/v0/financialEventGroups/{eventGroupId}/financialEvents | 
[**list_financial_events_by_order_id**](DefaultApi.md#list_financial_events_by_order_id) | **GET** /finances/v0/orders/{orderId}/financialEvents | 



## list_financial_event_groups

> crate::models::ListFinancialEventGroupsResponse list_financial_event_groups(max_results_per_page, financial_event_group_started_before, financial_event_group_started_after, next_token)


Returns financial event groups for a given date range.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results_per_page** | Option<**i32**> | The maximum number of results to return per page. If the response exceeds the maximum number of transactions or 10 MB, the API responds with 'InvalidInput'. |  |[default to 100]
**financial_event_group_started_before** | Option<**String**> | A date used for selecting financial event groups that opened before (but not at) a specified date and time, in ISO 8601 format. The date-time  must be later than FinancialEventGroupStartedAfter and no later than two minutes before the request was submitted. If FinancialEventGroupStartedAfter and FinancialEventGroupStartedBefore are more than 180 days apart, no financial event groups are returned. |  |
**financial_event_group_started_after** | Option<**String**> | A date used for selecting financial event groups that opened after (or at) a specified date and time, in ISO 8601 format. The date-time must be no later than two minutes before the request was submitted. |  |
**next_token** | Option<**String**> | A string token returned in the response of your previous request. |  |

### Return type

[**crate::models::ListFinancialEventGroupsResponse**](ListFinancialEventGroupsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_financial_events

> crate::models::ListFinancialEventsResponse list_financial_events(max_results_per_page, posted_after, posted_before, next_token)


Returns financial events for the specified data range.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results_per_page** | Option<**i32**> | The maximum number of results to return per page. If the response exceeds the maximum number of transactions or 10 MB, the API responds with 'InvalidInput'. |  |[default to 100]
**posted_after** | Option<**String**> | A date used for selecting financial events posted after (or at) a specified time. The date-time must be no later than two minutes before the request was submitted, in ISO 8601 date time format. |  |
**posted_before** | Option<**String**> | A date used for selecting financial events posted before (but not at) a specified time. The date-time must be later than PostedAfter and no later than two minutes before the request was submitted, in ISO 8601 date time format. If PostedAfter and PostedBefore are more than 180 days apart, no financial events are returned. You must specify the PostedAfter parameter if you specify the PostedBefore parameter. Default: Now minus two minutes. |  |
**next_token** | Option<**String**> | A string token returned in the response of your previous request. |  |

### Return type

[**crate::models::ListFinancialEventsResponse**](ListFinancialEventsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_financial_events_by_group_id

> crate::models::ListFinancialEventsResponse list_financial_events_by_group_id(event_group_id, max_results_per_page, posted_after, posted_before, next_token)


Returns all financial events for the specified financial event group.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_group_id** | **String** | The identifier of the financial event group to which the events belong. | [required] |
**max_results_per_page** | Option<**i32**> | The maximum number of results to return per page. If the response exceeds the maximum number of transactions or 10 MB, the API responds with 'InvalidInput'. |  |[default to 100]
**posted_after** | Option<**String**> | A date used for selecting financial events posted after (or at) a specified time. The date-time **must** be more than two minutes before the time of the request, in ISO 8601 date time format. |  |
**posted_before** | Option<**String**> | A date used for selecting financial events posted before (but not at) a specified time. The date-time must be later than `PostedAfter` and no later than two minutes before the request was submitted, in ISO 8601 date time format. If `PostedAfter` and `PostedBefore` are more than 180 days apart, no financial events are returned. You must specify the `PostedAfter` parameter if you specify the `PostedBefore` parameter. Default: Now minus two minutes. |  |
**next_token** | Option<**String**> | A string token returned in the response of your previous request. |  |

### Return type

[**crate::models::ListFinancialEventsResponse**](ListFinancialEventsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_financial_events_by_order_id

> crate::models::ListFinancialEventsResponse list_financial_events_by_order_id(order_id, max_results_per_page, next_token)


Returns all financial events for the specified order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | An Amazon-defined order identifier, in 3-7-7 format. | [required] |
**max_results_per_page** | Option<**i32**> | The maximum number of results to return per page. If the response exceeds the maximum number of transactions or 10 MB, the API responds with 'InvalidInput'. |  |[default to 100]
**next_token** | Option<**String**> | A string token returned in the response of your previous request. |  |

### Return type

[**crate::models::ListFinancialEventsResponse**](ListFinancialEventsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

