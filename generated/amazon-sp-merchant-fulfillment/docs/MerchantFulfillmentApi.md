# \MerchantFulfillmentApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_shipment**](MerchantFulfillmentApi.md#cancel_shipment) | **DELETE** /mfn/v0/shipments/{shipmentId} | 
[**cancel_shipment_old**](MerchantFulfillmentApi.md#cancel_shipment_old) | **PUT** /mfn/v0/shipments/{shipmentId}/cancel | 
[**create_shipment**](MerchantFulfillmentApi.md#create_shipment) | **POST** /mfn/v0/shipments | 
[**get_additional_seller_inputs**](MerchantFulfillmentApi.md#get_additional_seller_inputs) | **POST** /mfn/v0/additionalSellerInputs | 
[**get_additional_seller_inputs_old**](MerchantFulfillmentApi.md#get_additional_seller_inputs_old) | **POST** /mfn/v0/sellerInputs | 
[**get_eligible_shipment_services**](MerchantFulfillmentApi.md#get_eligible_shipment_services) | **POST** /mfn/v0/eligibleShippingServices | 
[**get_eligible_shipment_services_old**](MerchantFulfillmentApi.md#get_eligible_shipment_services_old) | **POST** /mfn/v0/eligibleServices | 
[**get_shipment**](MerchantFulfillmentApi.md#get_shipment) | **GET** /mfn/v0/shipments/{shipmentId} | 



## cancel_shipment

> crate::models::CancelShipmentResponse cancel_shipment(shipment_id)


Cancel the shipment indicated by the specified shipment identifier.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment_id** | **String** | The Amazon-defined shipment identifier for the shipment to cancel. | [required] |

### Return type

[**crate::models::CancelShipmentResponse**](CancelShipmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_shipment_old

> crate::models::CancelShipmentResponse cancel_shipment_old(shipment_id)


Cancel the shipment indicated by the specified shipment identifer.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment_id** | **String** | The Amazon-defined shipment identifier for the shipment to cancel. | [required] |

### Return type

[**crate::models::CancelShipmentResponse**](CancelShipmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_shipment

> crate::models::CreateShipmentResponse create_shipment(body)


Create a shipment with the information provided.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateShipmentRequest**](CreateShipmentRequest.md) |  | [required] |

### Return type

[**crate::models::CreateShipmentResponse**](CreateShipmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_additional_seller_inputs

> crate::models::GetAdditionalSellerInputsResponse get_additional_seller_inputs(body)


Gets a list of additional seller inputs required for a ship method. This is generally used for international shipping.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GetAdditionalSellerInputsRequest**](GetAdditionalSellerInputsRequest.md) |  | [required] |

### Return type

[**crate::models::GetAdditionalSellerInputsResponse**](GetAdditionalSellerInputsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_additional_seller_inputs_old

> crate::models::GetAdditionalSellerInputsResponse get_additional_seller_inputs_old(body)


Get a list of additional seller inputs required for a ship method. This is generally used for international shipping.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GetAdditionalSellerInputsRequest**](GetAdditionalSellerInputsRequest.md) |  | [required] |

### Return type

[**crate::models::GetAdditionalSellerInputsResponse**](GetAdditionalSellerInputsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eligible_shipment_services

> crate::models::GetEligibleShipmentServicesResponse get_eligible_shipment_services(body)


Returns a list of shipping service offers that satisfy the specified shipment request details.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GetEligibleShipmentServicesRequest**](GetEligibleShipmentServicesRequest.md) |  | [required] |

### Return type

[**crate::models::GetEligibleShipmentServicesResponse**](GetEligibleShipmentServicesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eligible_shipment_services_old

> crate::models::GetEligibleShipmentServicesResponse get_eligible_shipment_services_old(body)


Returns a list of shipping service offers that satisfy the specified shipment request details.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GetEligibleShipmentServicesRequest**](GetEligibleShipmentServicesRequest.md) |  | [required] |

### Return type

[**crate::models::GetEligibleShipmentServicesResponse**](GetEligibleShipmentServicesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shipment

> crate::models::GetShipmentResponse get_shipment(shipment_id)


Returns the shipment information for an existing shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment_id** | **String** | The Amazon-defined shipment identifier for the shipment. | [required] |

### Return type

[**crate::models::GetShipmentResponse**](GetShipmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

