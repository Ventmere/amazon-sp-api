# \ListingsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_listings_item**](ListingsApi.md#delete_listings_item) | **DELETE** /listings/2020-09-01/items/{sellerId}/{sku} | 
[**patch_listings_item**](ListingsApi.md#patch_listings_item) | **PATCH** /listings/2020-09-01/items/{sellerId}/{sku} | 
[**put_listings_item**](ListingsApi.md#put_listings_item) | **PUT** /listings/2020-09-01/items/{sellerId}/{sku} | 



## delete_listings_item

> crate::models::ListingsItemSubmissionResponse delete_listings_item(seller_id, sku, marketplace_ids, issue_locale)


Delete a listings item for a selling partner.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_id** | **String** | A selling partner identifier, such as a merchant account or vendor code. | [required] |
**sku** | **String** | A selling partner provided identifier for an Amazon listing. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**issue_locale** | Option<**String**> | A locale for localization of issues. When not provided, the default language code of the first marketplace is used. Examples: \"en_US\", \"fr_CA\", \"fr_FR\". Localized messages default to \"en_US\" when a localization is not available in the specified locale. |  |

### Return type

[**crate::models::ListingsItemSubmissionResponse**](ListingsItemSubmissionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_listings_item

> crate::models::ListingsItemSubmissionResponse patch_listings_item(seller_id, sku, marketplace_ids, body, issue_locale)


Partially update (patch) a listings item for a selling partner. Only top-level listings item attributes can be patched. Patching nested attributes is not supported.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_id** | **String** | A selling partner identifier, such as a merchant account or vendor code. | [required] |
**sku** | **String** | A selling partner provided identifier for an Amazon listing. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**body** | [**ListingsItemPatchRequest**](ListingsItemPatchRequest.md) | The request body schema for the patchListingsItem operation. | [required] |
**issue_locale** | Option<**String**> | A locale for localization of issues. When not provided, the default language code of the first marketplace is used. Examples: \"en_US\", \"fr_CA\", \"fr_FR\". Localized messages default to \"en_US\" when a localization is not available in the specified locale. |  |

### Return type

[**crate::models::ListingsItemSubmissionResponse**](ListingsItemSubmissionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_listings_item

> crate::models::ListingsItemSubmissionResponse put_listings_item(seller_id, sku, marketplace_ids, body, issue_locale)


Creates a new or fully-updates an existing listings item for a selling partner.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_id** | **String** | A selling partner identifier, such as a merchant account or vendor code. | [required] |
**sku** | **String** | A selling partner provided identifier for an Amazon listing. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**body** | [**ListingsItemPutRequest**](ListingsItemPutRequest.md) | The request body schema for the putListingsItem operation. | [required] |
**issue_locale** | Option<**String**> | A locale for localization of issues. When not provided, the default language code of the first marketplace is used. Examples: \"en_US\", \"fr_CA\", \"fr_FR\". Localized messages default to \"en_US\" when a localization is not available in the specified locale. |  |

### Return type

[**crate::models::ListingsItemSubmissionResponse**](ListingsItemSubmissionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

