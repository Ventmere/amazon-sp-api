# FulfillmentPreview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipping_speed_category** | [**crate::models::ShippingSpeedCategory**](ShippingSpeedCategory.md) |  | 
**scheduled_delivery_info** | Option<[**crate::models::ScheduledDeliveryInfo**](ScheduledDeliveryInfo.md)> |  | [optional]
**is_fulfillable** | **bool** | When true, this fulfillment order preview is fulfillable. | 
**is_cod_capable** | **bool** | When true, this fulfillment order preview is for COD (Cash On Delivery). | 
**estimated_shipping_weight** | Option<[**crate::models::Weight**](Weight.md)> |  | [optional]
**estimated_fees** | Option<[**Vec<crate::models::Fee>**](Fee.md)> | An array of fee type and cost pairs. | [optional]
**fulfillment_preview_shipments** | Option<[**Vec<crate::models::FulfillmentPreviewShipment>**](FulfillmentPreviewShipment.md)> | An array of fulfillment preview shipment information. | [optional]
**unfulfillable_preview_items** | Option<[**Vec<crate::models::UnfulfillablePreviewItem>**](UnfulfillablePreviewItem.md)> | An array of unfulfillable preview item information. | [optional]
**order_unfulfillable_reasons** | Option<**Vec<String>**> |  | [optional]
**marketplace_id** | **String** | The marketplace the fulfillment order is placed against. | 
**feature_constraints** | Option<[**Vec<crate::models::FeatureSettings>**](FeatureSettings.md)> | A list of features and their fulfillment policies to apply to the order. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


