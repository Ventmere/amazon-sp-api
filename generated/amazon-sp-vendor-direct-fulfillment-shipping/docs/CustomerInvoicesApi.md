# \CustomerInvoicesApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_customer_invoice**](CustomerInvoicesApi.md#get_customer_invoice) | **GET** /vendor/directFulfillment/shipping/v1/customerInvoices/{purchaseOrderNumber} | 
[**get_customer_invoices**](CustomerInvoicesApi.md#get_customer_invoices) | **GET** /vendor/directFulfillment/shipping/v1/customerInvoices | 



## get_customer_invoice

> crate::models::GetCustomerInvoiceResponse get_customer_invoice(purchase_order_number)


Returns a customer invoice based on the purchaseOrderNumber that you specify.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_number** | **String** | Purchase order number of the shipment for which to return the invoice. | [required] |

### Return type

[**crate::models::GetCustomerInvoiceResponse**](GetCustomerInvoiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_customer_invoices

> crate::models::GetCustomerInvoicesResponse get_customer_invoices(created_after, created_before, ship_from_party_id, limit, sort_order, next_token)


Returns a list of customer invoices created during a time frame that you specify. You define the  time frame using the createdAfter and createdBefore parameters. You must use both of these parameters. The date range to search must be no more than 7 days.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created_after** | **String** | Orders that became available after this date and time will be included in the result. Must be in ISO-8601 date/time format. | [required] |
**created_before** | **String** | Orders that became available before this date and time will be included in the result. Must be in ISO-8601 date/time format. | [required] |
**ship_from_party_id** | Option<**String**> | The vendor warehouseId for order fulfillment. If not specified, the result will contain orders for all warehouses. |  |
**limit** | Option<**i32**> | The limit to the number of records returned |  |
**sort_order** | Option<**String**> | Sort ASC or DESC by order creation date. |  |
**next_token** | Option<**String**> | Used for pagination when there are more orders than the specified result size limit. The token value is returned in the previous API call. |  |

### Return type

[**crate::models::GetCustomerInvoicesResponse**](GetCustomerInvoicesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, payload

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

