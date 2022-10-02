# \SmallAndLightApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_small_and_light_enrollment_by_seller_sku**](SmallAndLightApi.md#delete_small_and_light_enrollment_by_seller_sku) | **DELETE** /fba/smallAndLight/v1/enrollments/{sellerSKU} | 
[**get_small_and_light_eligibility_by_seller_sku**](SmallAndLightApi.md#get_small_and_light_eligibility_by_seller_sku) | **GET** /fba/smallAndLight/v1/eligibilities/{sellerSKU} | 
[**get_small_and_light_enrollment_by_seller_sku**](SmallAndLightApi.md#get_small_and_light_enrollment_by_seller_sku) | **GET** /fba/smallAndLight/v1/enrollments/{sellerSKU} | 
[**get_small_and_light_fee_preview**](SmallAndLightApi.md#get_small_and_light_fee_preview) | **POST** /fba/smallAndLight/v1/feePreviews | 
[**put_small_and_light_enrollment_by_seller_sku**](SmallAndLightApi.md#put_small_and_light_enrollment_by_seller_sku) | **PUT** /fba/smallAndLight/v1/enrollments/{sellerSKU} | 



## delete_small_and_light_enrollment_by_seller_sku

> delete_small_and_light_enrollment_by_seller_sku(seller_sku, marketplace_ids)


Removes the item indicated by the specified seller SKU from the Small and Light program in the specified marketplace. If the item is not eligible for disenrollment, the ineligibility reasons are returned.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU that identifies the item. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | The marketplace in which to remove the item from the Small and Light program. Note: Accepts a single marketplace only. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_small_and_light_eligibility_by_seller_sku

> crate::models::SmallAndLightEligibility get_small_and_light_eligibility_by_seller_sku(seller_sku, marketplace_ids)


Returns the Small and Light program eligibility status of the item indicated by the specified seller SKU in the specified marketplace. If the item is not eligible, the ineligibility reasons are returned.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 10 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU that identifies the item. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | The marketplace for which the eligibility status is retrieved. NOTE: Accepts a single marketplace only. | [required] |

### Return type

[**crate::models::SmallAndLightEligibility**](SmallAndLightEligibility.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_small_and_light_enrollment_by_seller_sku

> crate::models::SmallAndLightEnrollment get_small_and_light_enrollment_by_seller_sku(seller_sku, marketplace_ids)


Returns the Small and Light enrollment status for the item indicated by the specified seller SKU in the specified marketplace.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 10 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU that identifies the item. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | The marketplace for which the enrollment status is retrieved. Note: Accepts a single marketplace only. | [required] |

### Return type

[**crate::models::SmallAndLightEnrollment**](SmallAndLightEnrollment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_small_and_light_fee_preview

> crate::models::SmallAndLightFeePreviews get_small_and_light_fee_preview(body)


Returns the Small and Light fee estimates for the specified items. You must include a marketplaceId parameter to retrieve the proper fee estimates for items to be sold in that marketplace. The ordering of items in the response will mirror the order of the items in the request. Duplicate ASIN/price combinations are removed.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 3 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SmallAndLightFeePreviewRequest**](SmallAndLightFeePreviewRequest.md) |  | [required] |

### Return type

[**crate::models::SmallAndLightFeePreviews**](SmallAndLightFeePreviews.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_small_and_light_enrollment_by_seller_sku

> crate::models::SmallAndLightEnrollment put_small_and_light_enrollment_by_seller_sku(seller_sku, marketplace_ids)


Enrolls the item indicated by the specified seller SKU in the Small and Light program in the specified marketplace. If the item is not eligible, the ineligibility reasons are returned.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU that identifies the item. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | The marketplace in which to enroll the item. Note: Accepts a single marketplace only. | [required] |

### Return type

[**crate::models::SmallAndLightEnrollment**](SmallAndLightEnrollment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

