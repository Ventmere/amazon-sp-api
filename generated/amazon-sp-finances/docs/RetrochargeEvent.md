# RetrochargeEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**retrocharge_event_type** | Option<**String**> | The type of event.  Possible values:  * Retrocharge  * RetrochargeReversal | [optional]
**amazon_order_id** | Option<**String**> | An Amazon-defined identifier for an order. | [optional]
**posted_date** | Option<**String**> |  | [optional]
**base_tax** | Option<[**crate::models::Currency**](Currency.md)> |  | [optional]
**shipping_tax** | Option<[**crate::models::Currency**](Currency.md)> |  | [optional]
**marketplace_name** | Option<**String**> | The name of the marketplace where the retrocharge event occurred. | [optional]
**retrocharge_tax_withheld_list** | Option<[**Vec<crate::models::TaxWithheldComponent>**](TaxWithheldComponent.md)> | A list of information about taxes withheld. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


