# GetFulfillmentOrderResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fulfillment_order** | [**crate::models::FulfillmentOrder**](FulfillmentOrder.md) |  | 
**fulfillment_order_items** | [**Vec<crate::models::FulfillmentOrderItem>**](FulfillmentOrderItem.md) | An array of fulfillment order item information. | 
**fulfillment_shipments** | Option<[**Vec<crate::models::FulfillmentShipment>**](FulfillmentShipment.md)> | An array of fulfillment shipment information. | [optional]
**return_items** | [**Vec<crate::models::ReturnItem>**](ReturnItem.md) | An array of items that Amazon accepted for return. Returns empty if no items were accepted for return. | 
**return_authorizations** | [**Vec<crate::models::ReturnAuthorization>**](ReturnAuthorization.md) | An array of return authorization information. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


