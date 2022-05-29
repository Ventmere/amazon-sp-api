# Container

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_type** | **String** | The type of container. | 
**container_identifier** | **String** | The container identifier. | 
**tracking_number** | Option<**String**> | The tracking number. | [optional]
**manifest_id** | Option<**String**> | The manifest identifier. | [optional]
**manifest_date** | Option<**String**> | The date of the manifest. | [optional]
**ship_method** | Option<**String**> | The shipment method. | [optional]
**scac_code** | Option<**String**> | SCAC code required for NA VOC vendors only. | [optional]
**carrier** | Option<**String**> | Carrier required for EU VOC vendors only. | [optional]
**container_sequence_number** | Option<**i32**> | An integer that must be submitted for multi-box shipments only, where one item may come in separate packages. | [optional]
**dimensions** | Option<[**crate::models::Dimensions**](Dimensions.md)> |  | [optional]
**weight** | Option<[**crate::models::Weight**](Weight.md)> |  | [optional]
**packed_items** | [**Vec<crate::models::PackedItem>**](PackedItem.md) | A list of packed items. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


