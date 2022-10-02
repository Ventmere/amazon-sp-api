# InboundShipmentPlanRequestItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU of the item. | 
**ASIN** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | 
**condition** | [**crate::models::Condition**](Condition.md) |  | 
**quantity** | **i32** | The item quantity. | 
**quantity_in_case** | Option<**i32**> | The item quantity. | [optional]
**prep_details_list** | Option<[**Vec<crate::models::PrepDetails>**](PrepDetails.md)> | A list of preparation instructions and who is responsible for that preparation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


