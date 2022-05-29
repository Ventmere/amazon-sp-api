# \NotificationsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_destination**](NotificationsApi.md#create_destination) | **POST** /notifications/v1/destinations | 
[**create_subscription**](NotificationsApi.md#create_subscription) | **POST** /notifications/v1/subscriptions/{notificationType} | 
[**delete_destination**](NotificationsApi.md#delete_destination) | **DELETE** /notifications/v1/destinations/{destinationId} | 
[**delete_subscription_by_id**](NotificationsApi.md#delete_subscription_by_id) | **DELETE** /notifications/v1/subscriptions/{notificationType}/{subscriptionId} | 
[**get_destination**](NotificationsApi.md#get_destination) | **GET** /notifications/v1/destinations/{destinationId} | 
[**get_destinations**](NotificationsApi.md#get_destinations) | **GET** /notifications/v1/destinations | 
[**get_subscription**](NotificationsApi.md#get_subscription) | **GET** /notifications/v1/subscriptions/{notificationType} | 
[**get_subscription_by_id**](NotificationsApi.md#get_subscription_by_id) | **GET** /notifications/v1/subscriptions/{notificationType}/{subscriptionId} | 



## create_destination

> crate::models::CreateDestinationResponse create_destination(body)


Creates a destination resource to receive notifications. The createDestination API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateDestinationRequest**](CreateDestinationRequest.md) |  | [required] |

### Return type

[**crate::models::CreateDestinationResponse**](CreateDestinationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, Successful Response

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subscription

> crate::models::CreateSubscriptionResponse create_subscription(notification_type, body)


Creates a subscription for the specified notification type to be delivered to the specified destination. Before you can subscribe, you must first create the destination by calling the createDestination operation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_type** | **String** | The type of notification.   For more information about notification types, see [the Notifications API Use Case Guide](doc:notifications-api-v1-use-case-guide). | [required] |
**body** | [**CreateSubscriptionRequest**](CreateSubscriptionRequest.md) |  | [required] |

### Return type

[**crate::models::CreateSubscriptionResponse**](CreateSubscriptionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, Successful Response

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_destination

> crate::models::DeleteDestinationResponse delete_destination(destination_id)


Deletes the destination that you specify. The deleteDestination API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_id** | **String** | The identifier for the destination that you want to delete. | [required] |

### Return type

[**crate::models::DeleteDestinationResponse**](DeleteDestinationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, Successful Response

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription_by_id

> crate::models::DeleteSubscriptionByIdResponse delete_subscription_by_id(subscription_id, notification_type)


Deletes the subscription indicated by the subscription identifier and notification type that you specify. The subscription identifier can be for any subscription associated with your application. After you successfully call this operation, notifications will stop being sent for the associated subscription. The deleteSubscriptionById API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The identifier for the subscription that you want to delete. | [required] |
**notification_type** | **String** | The type of notification.   For more information about notification types, see [the Notifications API Use Case Guide](doc:notifications-api-v1-use-case-guide). | [required] |

### Return type

[**crate::models::DeleteSubscriptionByIdResponse**](DeleteSubscriptionByIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, Successful Operation Response

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_destination

> crate::models::GetDestinationResponse get_destination(destination_id)


Returns information about the destination that you specify. The getDestination API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_id** | **String** | The identifier generated when you created the destination. | [required] |

### Return type

[**crate::models::GetDestinationResponse**](GetDestinationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, Successful Response

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_destinations

> crate::models::GetDestinationsResponse get_destinations()


Returns information about all destinations. The getDestinations API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetDestinationsResponse**](GetDestinationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, Successful Response

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription

> crate::models::GetSubscriptionResponse get_subscription(notification_type)


Returns information about subscriptions of the specified notification type. You can use this API to get subscription information when you do not have a subscription identifier.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_type** | **String** | The type of notification.   For more information about notification types, see [the Notifications API Use Case Guide](doc:notifications-api-v1-use-case-guide). | [required] |

### Return type

[**crate::models::GetSubscriptionResponse**](GetSubscriptionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, Successful Response

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_by_id

> crate::models::GetSubscriptionByIdResponse get_subscription_by_id(subscription_id, notification_type)


Returns information about a subscription for the specified notification type. The getSubscriptionById API is grantless. For more information, see [Grantless operations](doc:grantless-operations) in the Selling Partner API Developer Guide.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The identifier for the subscription that you want to get. | [required] |
**notification_type** | **String** | The type of notification.   For more information about notification types, see [the Notifications API Use Case Guide](doc:notifications-api-v1-use-case-guide). | [required] |

### Return type

[**crate::models::GetSubscriptionByIdResponse**](GetSubscriptionByIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, Successful Response

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

