# \VendorShippingLabelsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_shipping_label**](VendorShippingLabelsApi.md#get_shipping_label) | **GET** /vendor/directFulfillment/shipping/v1/shippingLabels/{purchaseOrderNumber} | 
[**get_shipping_labels**](VendorShippingLabelsApi.md#get_shipping_labels) | **GET** /vendor/directFulfillment/shipping/v1/shippingLabels | 
[**submit_shipping_label_request**](VendorShippingLabelsApi.md#submit_shipping_label_request) | **POST** /vendor/directFulfillment/shipping/v1/shippingLabels | 



## get_shipping_label

> crate::models::GetShippingLabelResponse get_shipping_label(purchase_order_number)


Returns a shipping label for the purchaseOrderNumber that you specify.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The purchase order number for which you want to return the shipping label. It should be the same purchaseOrderNumber as received in the order. | [required] |

### Return type

[**crate::models::GetShippingLabelResponse**](GetShippingLabelResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shipping_labels

> crate::models::GetShippingLabelListResponse get_shipping_labels(created_after, created_before, ship_from_party_id, limit, sort_order, next_token)


Returns a list of shipping labels created during the time frame that you specify. You define that time frame using the createdAfter and createdBefore parameters. You must use both of these parameters. The date range to search must not be more than 7 days.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created_after** | **String** | Shipping labels that became available after this date and time will be included in the result. Must be in ISO-8601 date/time format. | [required] |
**created_before** | **String** | Shipping labels that became available before this date and time will be included in the result. Must be in ISO-8601 date/time format. | [required] |
**ship_from_party_id** | Option<**String**> | The vendor warehouseId for order fulfillment. If not specified, the result will contain orders for all warehouses. |  |
**limit** | Option<**i32**> | The limit to the number of records returned. |  |
**sort_order** | Option<**String**> | Sort ASC or DESC by order creation date. |  |[default to ASC]
**next_token** | Option<**String**> | Used for pagination when there are more ship labels than the specified result size limit. The token value is returned in the previous API call. |  |

### Return type

[**crate::models::GetShippingLabelListResponse**](GetShippingLabelListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, payload

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_shipping_label_request

> crate::models::SubmitShippingLabelsResponse submit_shipping_label_request(body)


Creates a shipping label for a purchase order and returns a transactionId for reference.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SubmitShippingLabelsRequest**](SubmitShippingLabelsRequest.md) |  | [required] |

### Return type

[**crate::models::SubmitShippingLabelsResponse**](SubmitShippingLabelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

