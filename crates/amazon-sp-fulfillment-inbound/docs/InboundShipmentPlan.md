# InboundShipmentPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | **String** | A shipment identifier originally returned by the createInboundShipmentPlan operation. | 
**destination_fulfillment_center_id** | **String** | An Amazon fulfillment center identifier created by Amazon. | 
**ship_to_address** | [**crate::models::Address**](Address.md) |  | 
**label_prep_type** | [**crate::models::LabelPrepType**](LabelPrepType.md) |  | 
**items** | [**Vec<crate::models::InboundShipmentPlanItem>**](InboundShipmentPlanItem.md) | A list of inbound shipment plan item information. | 
**estimated_box_contents_fee** | Option<[**crate::models::BoxContentsFeeDetails**](BoxContentsFeeDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


