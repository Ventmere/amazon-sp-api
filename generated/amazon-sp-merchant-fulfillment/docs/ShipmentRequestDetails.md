# ShipmentRequestDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon-defined order identifier, in 3-7-7 format. | 
**seller_order_id** | Option<**String**> | A seller-defined order identifier. | [optional]
**item_list** | [**Vec<crate::models::Item>**](Item.md) | The list of items to be included in a shipment. | 
**ship_from_address** | [**crate::models::Address**](Address.md) |  | 
**package_dimensions** | [**crate::models::PackageDimensions**](PackageDimensions.md) |  | 
**weight** | [**crate::models::Weight**](Weight.md) |  | 
**must_arrive_by_date** | Option<**String**> |  | [optional]
**ship_date** | Option<**String**> |  | [optional]
**shipping_service_options** | [**crate::models::ShippingServiceOptions**](ShippingServiceOptions.md) |  | 
**label_customization** | Option<[**crate::models::LabelCustomization**](LabelCustomization.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


