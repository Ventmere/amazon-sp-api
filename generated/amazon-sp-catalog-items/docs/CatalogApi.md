# \CatalogApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_catalog_item**](CatalogApi.md#get_catalog_item) | **GET** /catalog/v0/items/{asin} | 
[**list_catalog_categories**](CatalogApi.md#list_catalog_categories) | **GET** /catalog/v0/categories | 
[**list_catalog_items**](CatalogApi.md#list_catalog_items) | **GET** /catalog/v0/items | 



## get_catalog_item

> crate::models::GetCatalogItemResponse get_catalog_item(marketplace_id, asin)


Returns a specified item and its attributes.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | deprecated | deprecated |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. Specifies the marketplace for the item. | [required] |
**asin** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [required] |

### Return type

[**crate::models::GetCatalogItemResponse**](GetCatalogItemResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_catalog_categories

> crate::models::ListCatalogCategoriesResponse list_catalog_categories(marketplace_id, ASIN, seller_sku)


Returns the parent categories to which an item belongs, based on the specified ASIN or SellerSKU.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 2 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. Specifies the marketplace for the item. | [required] |
**ASIN** | Option<**String**> | The Amazon Standard Identification Number (ASIN) of the item. |  |
**seller_sku** | Option<**String**> | Used to identify items in the given marketplace. SellerSKU is qualified by the seller's SellerId, which is included with every operation that you submit. |  |

### Return type

[**crate::models::ListCatalogCategoriesResponse**](ListCatalogCategoriesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_catalog_items

> crate::models::ListCatalogItemsResponse list_catalog_items(marketplace_id, query, query_context_id, seller_sku, UPC, EAN, ISBN, JAN)


Returns a list of items and their attributes, based on a search query or item identifiers that you specify. When based on a search query, provide the Query parameter and optionally, the QueryContextId parameter. When based on item identifiers, provide a single appropriate parameter based on the identifier type, and specify the associated item value.  MarketplaceId is always required. At least one of Query, SellerSKU, UPC, EAN, ISBN, JAN is also required.  This operation returns a maximum of ten products and does not return non-buyable products.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | deprecated | deprecated |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. Specifies the marketplace for which items are returned. | [required] |
**query** | Option<**String**> | Keyword(s) to use to search for items in the catalog. Example: 'harry potter books'. |  |
**query_context_id** | Option<**String**> | An identifier for the context within which the given search will be performed. A marketplace might provide mechanisms for constraining a search to a subset of potential items. For example, the retail marketplace allows queries to be constrained to a specific category. The QueryContextId parameter specifies such a subset. If it is omitted, the search will be performed using the default context for the marketplace, which will typically contain the largest set of items. |  |
**seller_sku** | Option<**String**> | Used to identify an item in the given marketplace. SellerSKU is qualified by the seller's SellerId, which is included with every operation that you submit. |  |
**UPC** | Option<**String**> | A 12-digit bar code used for retail packaging. |  |
**EAN** | Option<**String**> | A European article number that uniquely identifies the catalog item, manufacturer, and its attributes. |  |
**ISBN** | Option<**String**> | The unique commercial book identifier used to identify books internationally. |  |
**JAN** | Option<**String**> | A Japanese article number that uniquely identifies the product, manufacturer, and its attributes. |  |

### Return type

[**crate::models::ListCatalogItemsResponse**](ListCatalogItemsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

