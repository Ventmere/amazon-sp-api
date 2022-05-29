# OrdersList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**orders** | [**Vec<crate::models::Order>**](Order.md) | A list of orders. | 
**next_token** | Option<**String**> | When present and not empty, pass this string token in the next request to return the next response page. | [optional]
**last_updated_before** | Option<**String**> | A date used for selecting orders that were last updated before (or at) a specified time. An update is defined as any change in order status, including the creation of a new order. Includes updates made by Amazon and by the seller. All dates must be in ISO 8601 format. | [optional]
**created_before** | Option<**String**> | A date used for selecting orders created before (or at) a specified time. Only orders placed before the specified time are returned. The date must be in ISO 8601 format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


