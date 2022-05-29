# \CatalogApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_catalog_item**](CatalogApi.md#get_catalog_item) | **GET** /catalog/2022-04-01/items/{asin} | 
[**search_catalog_items**](CatalogApi.md#search_catalog_items) | **GET** /catalog/2022-04-01/items | 



## get_catalog_item

> crate::models::Item get_catalog_item(asin, marketplace_ids, included_data, locale)


Retrieves details for an item in the Amazon catalog.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may observe higher rate and burst values than those shown here. For more information, refer to the [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asin** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers. Data sets in the response contain data only for the specified marketplaces. | [required] |
**included_data** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of data sets to include in the response. Default: `summaries`. |  |[default to ["summaries"]]
**locale** | Option<**String**> | Locale for retrieving localized summaries. Defaults to the primary locale of the marketplace. |  |

### Return type

[**crate::models::Item**](Item.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_catalog_items

> crate::models::ItemSearchResults search_catalog_items(marketplace_ids, identifiers, identifiers_type, included_data, locale, seller_id, keywords, brand_names, classification_ids, page_size, page_token, keywords_locale)


Search for and return a list of Amazon catalog items and associated information either by identifier or by keywords.  **Usage Plans:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may observe higher rate and burst values than those shown here. For more information, refer to the [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**identifiers** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of product identifiers to search the Amazon catalog for. **Note:** Cannot be used with `keywords`. |  |
**identifiers_type** | Option<**String**> | Type of product identifiers to search the Amazon catalog for. **Note:** Required when `identifiers` are provided. |  |
**included_data** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of data sets to include in the response. Default: `summaries`. |  |[default to ["summaries"]]
**locale** | Option<**String**> | Locale for retrieving localized summaries. Defaults to the primary locale of the marketplace. |  |
**seller_id** | Option<**String**> | A selling partner identifier, such as a seller account or vendor code. **Note:** Required when `identifiersType` is `SKU`. |  |
**keywords** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of words to search the Amazon catalog for. **Note:** Cannot be used with `identifiers`. |  |
**brand_names** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of brand names to limit the search for `keywords`-based queries. **Note:** Cannot be used with `identifiers`. |  |
**classification_ids** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of classification identifiers to limit the search for `keywords`-based queries. **Note:** Cannot be used with `identifiers`. |  |
**page_size** | Option<**i32**> | Number of results to be returned per page. |  |[default to 10]
**page_token** | Option<**String**> | A token to fetch a certain page when there are multiple pages worth of results. |  |
**keywords_locale** | Option<**String**> | The language of the keywords provided for `keywords`-based queries. Defaults to the primary locale of the marketplace. **Note:** Cannot be used with `identifiers`. |  |

### Return type

[**crate::models::ItemSearchResults**](ItemSearchResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

