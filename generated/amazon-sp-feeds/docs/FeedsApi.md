# \FeedsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_feed**](FeedsApi.md#cancel_feed) | **DELETE** /feeds/2020-09-04/feeds/{feedId} | 
[**create_feed**](FeedsApi.md#create_feed) | **POST** /feeds/2020-09-04/feeds | 
[**create_feed_document**](FeedsApi.md#create_feed_document) | **POST** /feeds/2020-09-04/documents | 
[**get_feed**](FeedsApi.md#get_feed) | **GET** /feeds/2020-09-04/feeds/{feedId} | 
[**get_feed_document**](FeedsApi.md#get_feed_document) | **GET** /feeds/2020-09-04/documents/{feedDocumentId} | 
[**get_feeds**](FeedsApi.md#get_feeds) | **GET** /feeds/2020-09-04/feeds | 



## cancel_feed

> crate::models::CancelFeedResponse cancel_feed(feed_id)


Cancels the feed that you specify. Only feeds with processingStatus=IN_QUEUE can be cancelled. Cancelled feeds are returned in subsequent calls to the getFeed and getFeeds operations.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0222 | 10 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | The identifier for the feed. This identifier is unique only in combination with a seller ID. | [required] |

### Return type

[**crate::models::CancelFeedResponse**](CancelFeedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_feed

> crate::models::CreateFeedResponse create_feed(body)


Creates a feed. Encrypt and upload the contents of the feed document before calling this operation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0083 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateFeedSpecification**](CreateFeedSpecification.md) |  | [required] |

### Return type

[**crate::models::CreateFeedResponse**](CreateFeedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_feed_document

> crate::models::CreateFeedDocumentResponse create_feed_document(body)


Creates a feed document for the feed type that you specify. This operation returns encryption details for encrypting the contents of the document, as well as a presigned URL for uploading the encrypted feed document contents. It also returns a feedDocumentId value that you can pass in with a subsequent call to the createFeed operation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0083 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateFeedDocumentSpecification**](CreateFeedDocumentSpecification.md) |  | [required] |

### Return type

[**crate::models::CreateFeedDocumentResponse**](CreateFeedDocumentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feed

> crate::models::GetFeedResponse get_feed(feed_id)


Returns feed details (including the resultDocumentId, if available) for the feed that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2.0 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | The identifier for the feed. This identifier is unique only in combination with a seller ID. | [required] |

### Return type

[**crate::models::GetFeedResponse**](GetFeedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feed_document

> crate::models::GetFeedDocumentResponse get_feed_document(feed_document_id)


Returns the information required for retrieving a feed document's contents. This includes a presigned URL for the feed document as well as the information required to decrypt the document's contents.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0222 | 10 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_document_id** | **String** | The identifier of the feed document. | [required] |

### Return type

[**crate::models::GetFeedDocumentResponse**](GetFeedDocumentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feeds

> crate::models::GetFeedsResponse get_feeds(feed_types, marketplace_ids, page_size, processing_statuses, created_since, created_until, next_token)


Returns feed details for the feeds that match the filters that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0222 | 10 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_types** | Option<[**Vec<String>**](String.md)> | A list of feed types used to filter feeds. When feedTypes is provided, the other filter parameters (processingStatuses, marketplaceIds, createdSince, createdUntil) and pageSize may also be provided. Either feedTypes or nextToken is required. |  |
**marketplace_ids** | Option<[**Vec<String>**](String.md)> | A list of marketplace identifiers used to filter feeds. The feeds returned will match at least one of the marketplaces that you specify. |  |
**page_size** | Option<**i32**> | The maximum number of feeds to return in a single call. |  |[default to 10]
**processing_statuses** | Option<[**Vec<String>**](String.md)> | A list of processing statuses used to filter feeds. |  |
**created_since** | Option<**String**> | The earliest feed creation date and time for feeds included in the response, in ISO 8601 format. The default is 90 days ago. Feeds are retained for a maximum of 90 days. |  |
**created_until** | Option<**String**> | The latest feed creation date and time for feeds included in the response, in ISO 8601 format. The default is now. |  |
**next_token** | Option<**String**> | A string token returned in the response to your previous request. nextToken is returned when the number of results exceeds the specified pageSize value. To get the next page of results, call the getFeeds operation and include this token as the only parameter. Specifying nextToken with any other parameters will cause the request to fail. |  |

### Return type

[**crate::models::GetFeedsResponse**](GetFeedsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

