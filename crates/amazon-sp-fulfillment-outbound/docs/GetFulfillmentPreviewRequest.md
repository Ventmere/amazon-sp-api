# GetFulfillmentPreviewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | Option<**String**> | The marketplace the fulfillment order is placed against. | [optional]
**address** | [**crate::models::Address**](Address.md) |  | 
**items** | [**Vec<crate::models::GetFulfillmentPreviewItem>**](GetFulfillmentPreviewItem.md) | An array of fulfillment preview item information. | 
**shipping_speed_categories** | Option<[**Vec<crate::models::ShippingSpeedCategory>**](ShippingSpeedCategory.md)> |  | [optional]
**include_cod_fulfillment_preview** | Option<**bool**> | Specifies whether to return fulfillment order previews that are for COD (Cash On Delivery).  Possible values:  * true - Returns all fulfillment order previews (both for COD and not for COD). * false - Returns only fulfillment order previews that are not for COD. | [optional]
**include_delivery_windows** | Option<**bool**> | Specifies whether to return the ScheduledDeliveryInfo response object, which contains the available delivery windows for a Scheduled Delivery. The ScheduledDeliveryInfo response object can only be returned for fulfillment order previews with ShippingSpeedCategories = ScheduledDelivery. | [optional]
**feature_constraints** | Option<[**Vec<crate::models::FeatureSettings>**](FeatureSettings.md)> | A list of features and their fulfillment policies to apply to the order. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


