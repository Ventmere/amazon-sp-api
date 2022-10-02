# \VendorShippingApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_packing_slip**](VendorShippingApi.md#get_packing_slip) | **GET** /vendor/directFulfillment/shipping/v1/packingSlips/{purchaseOrderNumber} | 
[**get_packing_slips**](VendorShippingApi.md#get_packing_slips) | **GET** /vendor/directFulfillment/shipping/v1/packingSlips | 
[**submit_shipment_confirmations**](VendorShippingApi.md#submit_shipment_confirmations) | **POST** /vendor/directFulfillment/shipping/v1/shipmentConfirmations | 
[**submit_shipment_status_updates**](VendorShippingApi.md#submit_shipment_status_updates) | **POST** /vendor/directFulfillment/shipping/v1/shipmentStatusUpdates | 



## get_packing_slip

> crate::models::GetPackingSlipResponse get_packing_slip(purchase_order_number)


Returns a packing slip based on the purchaseOrderNumber that you specify.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The purchaseOrderNumber for the packing slip you want. | [required] |

### Return type

[**crate::models::GetPackingSlipResponse**](GetPackingSlipResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_packing_slips

> crate::models::GetPackingSlipListResponse get_packing_slips(created_after, created_before, ship_from_party_id, limit, sort_order, next_token)


Returns a list of packing slips for the purchase orders that match the criteria specified. Date range to search must not be more than 7 days.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created_after** | **String** | Packing slips that became available after this date and time will be included in the result. Must be in ISO-8601 date/time format. | [required] |
**created_before** | **String** | Packing slips that became available before this date and time will be included in the result. Must be in ISO-8601 date/time format. | [required] |
**ship_from_party_id** | Option<**String**> | The vendor warehouseId for order fulfillment. If not specified the result will contain orders for all warehouses. |  |
**limit** | Option<**i32**> | The limit to the number of records returned |  |
**sort_order** | Option<**String**> | Sort ASC or DESC by packing slip creation date. |  |[default to ASC]
**next_token** | Option<**String**> | Used for pagination when there are more packing slips than the specified result size limit. The token value is returned in the previous API call. |  |

### Return type

[**crate::models::GetPackingSlipListResponse**](GetPackingSlipListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_shipment_confirmations

> crate::models::SubmitShipmentConfirmationsResponse submit_shipment_confirmations(body)


Submits one or more shipment confirmations for vendor orders.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SubmitShipmentConfirmationsRequest**](SubmitShipmentConfirmationsRequest.md) |  | [required] |

### Return type

[**crate::models::SubmitShipmentConfirmationsResponse**](SubmitShipmentConfirmationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_shipment_status_updates

> crate::models::SubmitShipmentStatusUpdatesResponse submit_shipment_status_updates(body)


This API call is only to be used by Vendor-Own-Carrier (VOC) vendors. Calling this API will submit a shipment status update for the package that a vendor has shipped. It will provide the Amazon customer visibility on their order, when the package is outside of Amazon Network visibility.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SubmitShipmentStatusUpdatesRequest**](SubmitShipmentStatusUpdatesRequest.md) |  | [required] |

### Return type

[**crate::models::SubmitShipmentStatusUpdatesResponse**](SubmitShipmentStatusUpdatesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

