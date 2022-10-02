# ShippingServiceOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delivery_experience** | [**crate::models::DeliveryExperienceType**](DeliveryExperienceType.md) |  | 
**declared_value** | Option<[**crate::models::CurrencyAmount**](CurrencyAmount.md)> |  | [optional]
**carrier_will_pick_up** | **bool** | When true, the carrier will pick up the package.  Note: Scheduled carrier pickup is available only using Dynamex (US), DPD (UK), and Royal Mail (UK). | 
**carrier_will_pick_up_option** | Option<[**crate::models::CarrierWillPickUpOption**](CarrierWillPickUpOption.md)> |  | [optional]
**label_format** | Option<[**crate::models::LabelFormat**](LabelFormat.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


