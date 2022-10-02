# Shipment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | **String** | An Amazon-defined shipment identifier. | 
**amazon_order_id** | **String** | An Amazon-defined order identifier, in 3-7-7 format. | 
**seller_order_id** | Option<**String**> | A seller-defined order identifier. | [optional]
**item_list** | [**Vec<crate::models::Item>**](Item.md) | The list of items to be included in a shipment. | 
**ship_from_address** | [**crate::models::Address**](Address.md) |  | 
**ship_to_address** | [**crate::models::Address**](Address.md) |  | 
**package_dimensions** | [**crate::models::PackageDimensions**](PackageDimensions.md) |  | 
**weight** | [**crate::models::Weight**](Weight.md) |  | 
**insurance** | [**crate::models::CurrencyAmount**](CurrencyAmount.md) |  | 
**shipping_service** | [**crate::models::ShippingService**](ShippingService.md) |  | 
**label** | [**crate::models::Label**](Label.md) |  | 
**status** | [**crate::models::ShipmentStatus**](ShipmentStatus.md) |  | 
**tracking_id** | Option<**String**> | The shipment tracking identifier provided by the carrier. | [optional]
**created_date** | **String** |  | 
**last_updated_date** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


