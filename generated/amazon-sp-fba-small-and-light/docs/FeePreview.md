# FeePreview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | Option<**String**> | The Amazon Standard Identification Number (ASIN) value used to identify the item. | [optional]
**price** | Option<[**crate::models::MoneyType**](MoneyType.md)> |  | [optional]
**fee_breakdown** | Option<[**Vec<crate::models::FeeLineItem>**](FeeLineItem.md)> | A list of the Small and Light fees for the item. | [optional]
**total_fees** | Option<[**crate::models::MoneyType**](MoneyType.md)> |  | [optional]
**errors** | Option<[**Vec<crate::models::Error>**](Error.md)> | One or more unexpected errors occurred during the getSmallAndLightFeePreview operation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


