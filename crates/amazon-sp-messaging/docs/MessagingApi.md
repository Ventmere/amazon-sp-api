# \MessagingApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirm_customization_details**](MessagingApi.md#confirm_customization_details) | **POST** /messaging/v1/orders/{amazonOrderId}/messages/confirmCustomizationDetails | 
[**create_amazon_motors**](MessagingApi.md#create_amazon_motors) | **POST** /messaging/v1/orders/{amazonOrderId}/messages/amazonMotors | 
[**create_confirm_delivery_details**](MessagingApi.md#create_confirm_delivery_details) | **POST** /messaging/v1/orders/{amazonOrderId}/messages/confirmDeliveryDetails | 
[**create_confirm_order_details**](MessagingApi.md#create_confirm_order_details) | **POST** /messaging/v1/orders/{amazonOrderId}/messages/confirmOrderDetails | 
[**create_confirm_service_details**](MessagingApi.md#create_confirm_service_details) | **POST** /messaging/v1/orders/{amazonOrderId}/messages/confirmServiceDetails | 
[**create_digital_access_key**](MessagingApi.md#create_digital_access_key) | **POST** /messaging/v1/orders/{amazonOrderId}/messages/digitalAccessKey | 
[**create_legal_disclosure**](MessagingApi.md#create_legal_disclosure) | **POST** /messaging/v1/orders/{amazonOrderId}/messages/legalDisclosure | 
[**create_negative_feedback_removal**](MessagingApi.md#create_negative_feedback_removal) | **POST** /messaging/v1/orders/{amazonOrderId}/messages/negativeFeedbackRemoval | 
[**create_unexpected_problem**](MessagingApi.md#create_unexpected_problem) | **POST** /messaging/v1/orders/{amazonOrderId}/messages/unexpectedProblem | 
[**create_warranty**](MessagingApi.md#create_warranty) | **POST** /messaging/v1/orders/{amazonOrderId}/messages/warranty | 
[**get_attributes**](MessagingApi.md#get_attributes) | **GET** /messaging/v1/orders/{amazonOrderId}/attributes | 
[**get_messaging_actions_for_order**](MessagingApi.md#get_messaging_actions_for_order) | **GET** /messaging/v1/orders/{amazonOrderId} | 



## confirm_customization_details

> crate::models::CreateConfirmCustomizationDetailsResponse confirm_customization_details(amazon_order_id, marketplace_ids, body)


Sends a message asking a buyer to provide or verify customization details such as name spelling, images, initials, etc.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |
**body** | [**CreateConfirmCustomizationDetailsRequest**](CreateConfirmCustomizationDetailsRequest.md) |  | [required] |

### Return type

[**crate::models::CreateConfirmCustomizationDetailsResponse**](CreateConfirmCustomizationDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_amazon_motors

> crate::models::CreateAmazonMotorsResponse create_amazon_motors(amazon_order_id, marketplace_ids, body)


Sends a message to a buyer to provide details about an Amazon Motors order. This message can only be sent by Amazon Motors sellers.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |
**body** | [**CreateAmazonMotorsRequest**](CreateAmazonMotorsRequest.md) |  | [required] |

### Return type

[**crate::models::CreateAmazonMotorsResponse**](CreateAmazonMotorsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_confirm_delivery_details

> crate::models::CreateConfirmDeliveryDetailsResponse create_confirm_delivery_details(amazon_order_id, marketplace_ids, body)


Sends a message to a buyer to arrange a delivery or to confirm contact information for making a delivery.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |
**body** | [**CreateConfirmDeliveryDetailsRequest**](CreateConfirmDeliveryDetailsRequest.md) |  | [required] |

### Return type

[**crate::models::CreateConfirmDeliveryDetailsResponse**](CreateConfirmDeliveryDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_confirm_order_details

> crate::models::CreateConfirmOrderDetailsResponse create_confirm_order_details(amazon_order_id, marketplace_ids, body)


Sends a message to ask a buyer an order-related question prior to shipping their order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |
**body** | [**CreateConfirmOrderDetailsRequest**](CreateConfirmOrderDetailsRequest.md) |  | [required] |

### Return type

[**crate::models::CreateConfirmOrderDetailsResponse**](CreateConfirmOrderDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_confirm_service_details

> crate::models::CreateConfirmServiceDetailsResponse create_confirm_service_details(amazon_order_id, marketplace_ids, body)


Sends a message to contact a Home Service customer to arrange a service call or to gather information prior to a service call.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |
**body** | [**CreateConfirmServiceDetailsRequest**](CreateConfirmServiceDetailsRequest.md) |  | [required] |

### Return type

[**crate::models::CreateConfirmServiceDetailsResponse**](CreateConfirmServiceDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_digital_access_key

> crate::models::CreateDigitalAccessKeyResponse create_digital_access_key(amazon_order_id, marketplace_ids, body)


Sends a message to a buyer to share a digital access key needed to utilize digital content in their order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |
**body** | [**CreateDigitalAccessKeyRequest**](CreateDigitalAccessKeyRequest.md) |  | [required] |

### Return type

[**crate::models::CreateDigitalAccessKeyResponse**](CreateDigitalAccessKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_legal_disclosure

> crate::models::CreateLegalDisclosureResponse create_legal_disclosure(amazon_order_id, marketplace_ids, body)


Sends a critical message that contains documents that a seller is legally obligated to provide to the buyer. This message should only be used to deliver documents that are required by law.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |
**body** | [**CreateLegalDisclosureRequest**](CreateLegalDisclosureRequest.md) |  | [required] |

### Return type

[**crate::models::CreateLegalDisclosureResponse**](CreateLegalDisclosureResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_negative_feedback_removal

> crate::models::CreateNegativeFeedbackRemovalResponse create_negative_feedback_removal(amazon_order_id, marketplace_ids)


Sends a non-critical message that asks a buyer to remove their negative feedback. This message should only be sent after the seller has resolved the buyer's problem.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |

### Return type

[**crate::models::CreateNegativeFeedbackRemovalResponse**](CreateNegativeFeedbackRemovalResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_unexpected_problem

> crate::models::CreateUnexpectedProblemResponse create_unexpected_problem(amazon_order_id, marketplace_ids, body)


Sends a critical message to a buyer that an unexpected problem was encountered affecting the completion of the order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |
**body** | [**CreateUnexpectedProblemRequest**](CreateUnexpectedProblemRequest.md) |  | [required] |

### Return type

[**crate::models::CreateUnexpectedProblemResponse**](CreateUnexpectedProblemResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_warranty

> crate::models::CreateWarrantyResponse create_warranty(amazon_order_id, marketplace_ids, body)


Sends a message to a buyer to provide details about warranty information on a purchase in their order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |
**body** | [**CreateWarrantyRequest**](CreateWarrantyRequest.md) |  | [required] |

### Return type

[**crate::models::CreateWarrantyResponse**](CreateWarrantyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attributes

> crate::models::GetAttributesResponse get_attributes(amazon_order_id, marketplace_ids)


Returns a response containing attributes related to an order. This includes buyer preferences.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a message is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |

### Return type

[**crate::models::GetAttributesResponse**](GetAttributesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_messaging_actions_for_order

> crate::models::GetMessagingActionsForOrderResponse get_messaging_actions_for_order(amazon_order_id, marketplace_ids)


Returns a list of message types that are available for an order that you specify. A message type is represented by an actions object, which contains a path and query parameter(s). You can use the path and parameter(s) to call an operation that sends a message.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which you want a list of available message types. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |

### Return type

[**crate::models::GetMessagingActionsForOrderResponse**](GetMessagingActionsForOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

