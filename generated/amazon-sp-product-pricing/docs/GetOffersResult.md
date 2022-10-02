# GetOffersResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. | 
**ASIN** | Option<**String**> | The Amazon Standard Identification Number (ASIN) of the item. | [optional]
**SKU** | Option<**String**> | The stock keeping unit (SKU) of the item. | [optional]
**item_condition** | [**crate::models::ConditionType**](ConditionType.md) |  | 
**status** | **String** | The status of the operation. | 
**identifier** | [**crate::models::ItemIdentifier**](ItemIdentifier.md) |  | 
**summary** | [**crate::models::Summary**](Summary.md) |  | 
**offers** | [**Vec<crate::models::OfferDetail>**](OfferDetail.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


