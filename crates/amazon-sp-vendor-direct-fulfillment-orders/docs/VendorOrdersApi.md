# \VendorOrdersApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_order**](VendorOrdersApi.md#get_order) | **GET** /vendor/directFulfillment/orders/2021-12-28/purchaseOrders/{purchaseOrderNumber} | 
[**get_orders**](VendorOrdersApi.md#get_orders) | **GET** /vendor/directFulfillment/orders/2021-12-28/purchaseOrders | 
[**submit_acknowledgement**](VendorOrdersApi.md#submit_acknowledgement) | **POST** /vendor/directFulfillment/orders/2021-12-28/acknowledgements | 



## get_order

> crate::models::Order get_order(purchase_order_number)


Returns purchase order information for the purchaseOrderNumber that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The order identifier for the purchase order that you want. Formatting Notes: alpha-numeric code. | [required] |

### Return type

[**crate::models::Order**](Order.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orders

> crate::models::OrderList get_orders(created_after, created_before, ship_from_party_id, status, limit, sort_order, next_token, include_details)


Returns a list of purchase orders created during the time frame that you specify. You define the time frame using the createdAfter and createdBefore parameters. You must use both parameters. You can choose to get only the purchase order numbers by setting the includeDetails parameter to false. In that case, the operation returns a list of purchase order numbers. You can then call the getOrder operation to return the details of a specific order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created_after** | **String** | Purchase orders that became available after this date and time will be included in the result. Must be in ISO-8601 date/time format. | [required] |
**created_before** | **String** | Purchase orders that became available before this date and time will be included in the result. Must be in ISO-8601 date/time format. | [required] |
**ship_from_party_id** | Option<**String**> | The vendor warehouse identifier for the fulfillment warehouse. If not specified, the result will contain orders for all warehouses. |  |
**status** | Option<**String**> | Returns only the purchase orders that match the specified status. If not specified, the result will contain orders that match any status. |  |
**limit** | Option<**i64**> | The limit to the number of purchase orders returned. |  |
**sort_order** | Option<**String**> | Sort the list in ascending or descending order by order creation date. |  |
**next_token** | Option<**String**> | Used for pagination when there are more orders than the specified result size limit. The token value is returned in the previous API call. |  |
**include_details** | Option<**bool**> | When true, returns the complete purchase order details. Otherwise, only purchase order numbers are returned. |  |[default to true]

### Return type

[**crate::models::OrderList**](OrderList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, pagination, orders

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_acknowledgement

> crate::models::TransactionId submit_acknowledgement(body)


Submits acknowledgements for one or more purchase orders.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SubmitAcknowledgementRequest**](SubmitAcknowledgementRequest.md) |  | [required] |

### Return type

[**crate::models::TransactionId**](TransactionId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

