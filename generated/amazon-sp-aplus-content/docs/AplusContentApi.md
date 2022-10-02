# \AplusContentApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_content_document**](AplusContentApi.md#create_content_document) | **POST** /aplus/2020-11-01/contentDocuments | 
[**get_content_document**](AplusContentApi.md#get_content_document) | **GET** /aplus/2020-11-01/contentDocuments/{contentReferenceKey} | 
[**list_content_document_asin_relations**](AplusContentApi.md#list_content_document_asin_relations) | **GET** /aplus/2020-11-01/contentDocuments/{contentReferenceKey}/asins | 
[**post_content_document_approval_submission**](AplusContentApi.md#post_content_document_approval_submission) | **POST** /aplus/2020-11-01/contentDocuments/{contentReferenceKey}/approvalSubmissions | 
[**post_content_document_asin_relations**](AplusContentApi.md#post_content_document_asin_relations) | **POST** /aplus/2020-11-01/contentDocuments/{contentReferenceKey}/asins | 
[**post_content_document_suspend_submission**](AplusContentApi.md#post_content_document_suspend_submission) | **POST** /aplus/2020-11-01/contentDocuments/{contentReferenceKey}/suspendSubmissions | 
[**search_content_documents**](AplusContentApi.md#search_content_documents) | **GET** /aplus/2020-11-01/contentDocuments | 
[**search_content_publish_records**](AplusContentApi.md#search_content_publish_records) | **GET** /aplus/2020-11-01/contentPublishRecords | 
[**update_content_document**](AplusContentApi.md#update_content_document) | **POST** /aplus/2020-11-01/contentDocuments/{contentReferenceKey} | 
[**validate_content_document_asin_relations**](AplusContentApi.md#validate_content_document_asin_relations) | **POST** /aplus/2020-11-01/contentAsinValidations | 



## create_content_document

> crate::models::PostContentDocumentResponse create_content_document(marketplace_id, post_content_document_request)


Creates a new A+ Content document.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | The identifier for the marketplace where the A+ Content is published. | [required] |
**post_content_document_request** | [**PostContentDocumentRequest**](PostContentDocumentRequest.md) | The content document request details. | [required] |

### Return type

[**crate::models::PostContentDocumentResponse**](PostContentDocumentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_content_document

> crate::models::GetContentDocumentResponse get_content_document(content_reference_key, marketplace_id, included_data_set)


Returns an A+ Content document, if available.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_reference_key** | **String** | The unique reference key for the A+ Content document. A content reference key cannot form a permalink and may change in the future. A content reference key is not guaranteed to match any A+ Content identifier. | [required] |
**marketplace_id** | **String** | The identifier for the marketplace where the A+ Content is published. | [required] |
**included_data_set** | [**Vec<String>**](String.md) | The set of A+ Content data types to include in the response. | [required] |

### Return type

[**crate::models::GetContentDocumentResponse**](GetContentDocumentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_content_document_asin_relations

> crate::models::ListContentDocumentAsinRelationsResponse list_content_document_asin_relations(content_reference_key, marketplace_id, included_data_set, asin_set, page_token)


Returns a list of ASINs related to the specified A+ Content document, if available. If you do not include the asinSet parameter, the operation returns all ASINs related to the content document.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_reference_key** | **String** | The unique reference key for the A+ Content document. A content reference key cannot form a permalink and may change in the future. A content reference key is not guaranteed to match any A+ Content identifier. | [required] |
**marketplace_id** | **String** | The identifier for the marketplace where the A+ Content is published. | [required] |
**included_data_set** | Option<[**Vec<String>**](String.md)> | The set of A+ Content data types to include in the response. If you do not include this parameter, the operation returns the related ASINs without metadata. |  |
**asin_set** | Option<[**Vec<String>**](String.md)> | The set of ASINs. |  |
**page_token** | Option<**String**> | A page token from the nextPageToken response element returned by your previous call to this operation. nextPageToken is returned when the results of a call exceed the page size. To get the next page of results, call the operation and include pageToken as the only parameter. Specifying pageToken with any other parameter will cause the request to fail. When no nextPageToken value is returned there are no more pages to return. A pageToken value is not usable across different operations. |  |

### Return type

[**crate::models::ListContentDocumentAsinRelationsResponse**](ListContentDocumentAsinRelationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_content_document_approval_submission

> crate::models::PostContentDocumentApprovalSubmissionResponse post_content_document_approval_submission(content_reference_key, marketplace_id)


Submits an A+ Content document for review, approval, and publishing.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_reference_key** | **String** | The unique reference key for the A+ Content document. A content reference key cannot form a permalink and may change in the future. A content reference key is not guaranteed to match any A+ content identifier. | [required] |
**marketplace_id** | **String** | The identifier for the marketplace where the A+ Content is published. | [required] |

### Return type

[**crate::models::PostContentDocumentApprovalSubmissionResponse**](PostContentDocumentApprovalSubmissionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_content_document_asin_relations

> crate::models::PostContentDocumentAsinRelationsResponse post_content_document_asin_relations(content_reference_key, marketplace_id, post_content_document_asin_relations_request)


Replaces all ASINs related to the specified A+ Content document, if available. This may add or remove ASINs, depending on the current set of related ASINs. Removing an ASIN has the side effect of suspending the content document from that ASIN.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_reference_key** | **String** | The unique reference key for the A+ Content document. A content reference key cannot form a permalink and may change in the future. A content reference key is not guaranteed to match any A+ content identifier. | [required] |
**marketplace_id** | **String** | The identifier for the marketplace where the A+ Content is published. | [required] |
**post_content_document_asin_relations_request** | [**PostContentDocumentAsinRelationsRequest**](PostContentDocumentAsinRelationsRequest.md) | The content document ASIN relations request details. | [required] |

### Return type

[**crate::models::PostContentDocumentAsinRelationsResponse**](PostContentDocumentAsinRelationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_content_document_suspend_submission

> crate::models::PostContentDocumentSuspendSubmissionResponse post_content_document_suspend_submission(content_reference_key, marketplace_id)


Submits a request to suspend visible A+ Content. This neither deletes the content document nor the ASIN relations.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_reference_key** | **String** | The unique reference key for the A+ Content document. A content reference key cannot form a permalink and may change in the future. A content reference key is not guaranteed to match any A+ content identifier. | [required] |
**marketplace_id** | **String** | The identifier for the marketplace where the A+ Content is published. | [required] |

### Return type

[**crate::models::PostContentDocumentSuspendSubmissionResponse**](PostContentDocumentSuspendSubmissionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_content_documents

> crate::models::SearchContentDocumentsResponse search_content_documents(marketplace_id, page_token)


Returns a list of all A+ Content documents assigned to a selling partner. This operation returns only the metadata of the A+ Content documents. Call the getContentDocument operation to get the actual contents of the A+ Content documents.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | The identifier for the marketplace where the A+ Content is published. | [required] |
**page_token** | Option<**String**> | A page token from the nextPageToken response element returned by your previous call to this operation. nextPageToken is returned when the results of a call exceed the page size. To get the next page of results, call the operation and include pageToken as the only parameter. Specifying pageToken with any other parameter will cause the request to fail. When no nextPageToken value is returned there are no more pages to return. A pageToken value is not usable across different operations. |  |

### Return type

[**crate::models::SearchContentDocumentsResponse**](SearchContentDocumentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_content_publish_records

> crate::models::SearchContentPublishRecordsResponse search_content_publish_records(marketplace_id, asin, page_token)


Searches for A+ Content publishing records, if available.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | The identifier for the marketplace where the A+ Content is published. | [required] |
**asin** | **String** | The Amazon Standard Identification Number (ASIN). | [required] |
**page_token** | Option<**String**> | A page token from the nextPageToken response element returned by your previous call to this operation. nextPageToken is returned when the results of a call exceed the page size. To get the next page of results, call the operation and include pageToken as the only parameter. Specifying pageToken with any other parameter will cause the request to fail. When no nextPageToken value is returned there are no more pages to return. A pageToken value is not usable across different operations. |  |

### Return type

[**crate::models::SearchContentPublishRecordsResponse**](SearchContentPublishRecordsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_content_document

> crate::models::PostContentDocumentResponse update_content_document(content_reference_key, marketplace_id, post_content_document_request)


Updates an existing A+ Content document.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_reference_key** | **String** | The unique reference key for the A+ Content document. A content reference key cannot form a permalink and may change in the future. A content reference key is not guaranteed to match any A+ Content identifier. | [required] |
**marketplace_id** | **String** | The identifier for the marketplace where the A+ Content is published. | [required] |
**post_content_document_request** | [**PostContentDocumentRequest**](PostContentDocumentRequest.md) | The content document request details. | [required] |

### Return type

[**crate::models::PostContentDocumentResponse**](PostContentDocumentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_content_document_asin_relations

> crate::models::ValidateContentDocumentAsinRelationsResponse validate_content_document_asin_relations(marketplace_id, post_content_document_request, asin_set)


Checks if the A+ Content document is valid for use on a set of ASINs.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | The identifier for the marketplace where the A+ Content is published. | [required] |
**post_content_document_request** | [**PostContentDocumentRequest**](PostContentDocumentRequest.md) | The content document request details. | [required] |
**asin_set** | Option<[**Vec<String>**](String.md)> | The set of ASINs. |  |

### Return type

[**crate::models::ValidateContentDocumentAsinRelationsResponse**](ValidateContentDocumentAsinRelationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

