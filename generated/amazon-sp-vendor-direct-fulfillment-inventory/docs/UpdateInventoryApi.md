# \UpdateInventoryApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**submit_inventory_update**](UpdateInventoryApi.md#submit_inventory_update) | **POST** /vendor/directFulfillment/inventory/v1/warehouses/{warehouseId}/items | 



## submit_inventory_update

> crate::models::SubmitInventoryUpdateResponse submit_inventory_update(warehouse_id, body)


Submits inventory updates for the specified warehouse for either a partial or full feed of inventory items.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | **String** | Identifier for the warehouse for which to update inventory. | [required] |
**body** | [**SubmitInventoryUpdateRequest**](SubmitInventoryUpdateRequest.md) |  | [required] |

### Return type

[**crate::models::SubmitInventoryUpdateResponse**](SubmitInventoryUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

