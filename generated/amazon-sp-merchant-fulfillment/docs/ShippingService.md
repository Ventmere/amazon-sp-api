# ShippingService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipping_service_name** | **String** | A plain text representation of a carrier's shipping service. For example, \"UPS Ground\" or \"FedEx Standard Overnight\".  | 
**carrier_name** | **String** | The name of the carrier. | 
**shipping_service_id** | **String** | An Amazon-defined shipping service identifier. | 
**shipping_service_offer_id** | **String** | An Amazon-defined shipping service offer identifier. | 
**ship_date** | **String** |  | 
**earliest_estimated_delivery_date** | Option<**String**> |  | [optional]
**latest_estimated_delivery_date** | Option<**String**> |  | [optional]
**rate** | [**crate::models::CurrencyAmount**](CurrencyAmount.md) |  | 
**shipping_service_options** | [**crate::models::ShippingServiceOptions**](ShippingServiceOptions.md) |  | 
**available_shipping_service_options** | Option<[**crate::models::AvailableShippingServiceOptions**](AvailableShippingServiceOptions.md)> |  | [optional]
**available_label_formats** | Option<[**Vec<crate::models::LabelFormat>**](LabelFormat.md)> | List of label formats. | [optional]
**available_format_options_for_label** | Option<[**Vec<crate::models::LabelFormatOption>**](LabelFormatOption.md)> | The available label formats. | [optional]
**requires_additional_seller_inputs** | **bool** | When true, additional seller inputs are required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


