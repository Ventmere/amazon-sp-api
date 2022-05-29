# UpdateFulfillmentOrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | Option<**String**> | The seller SKU of the item. | [optional]
**seller_fulfillment_order_item_id** | **String** | Identifies the fulfillment order item to update. Created with a previous call to the createFulfillmentOrder operation. | 
**quantity** | **i32** | The item quantity. | 
**gift_message** | Option<**String**> | A message to the gift recipient, if applicable. | [optional]
**displayable_comment** | Option<**String**> | Item-specific text that displays in recipient-facing materials such as the outbound shipment packing slip. | [optional]
**fulfillment_network_sku** | Option<**String**> | Amazon's fulfillment network SKU of the item. | [optional]
**order_item_disposition** | Option<**String**> | Indicates whether the item is sellable or unsellable. | [optional]
**per_unit_declared_value** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**per_unit_price** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**per_unit_tax** | Option<[**crate::models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


