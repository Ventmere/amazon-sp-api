# ItemSummaryByMarketplace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. Identifies the Amazon marketplace for the listings item. | 
**asin** | **String** | Amazon Standard Identification Number (ASIN) of the listings item. | 
**product_type** | **String** | The Amazon product type of the listings item. | 
**condition_type** | Option<**String**> | Identifies the condition of the listings item. | [optional]
**status** | **Vec<String>** | Statuses that apply to the listings item. | 
**fn_sku** | Option<**String**> | Fulfillment network stock keeping unit is an identifier used by Amazon fulfillment centers to identify each unique item. | [optional]
**item_name** | **String** | Name, or title, associated with an Amazon catalog item. | 
**created_date** | **String** | Date the listings item was created, in ISO 8601 format. | 
**last_updated_date** | **String** | Date the listings item was last updated, in ISO 8601 format. | 
**main_image** | Option<[**crate::models::ItemImage**](ItemImage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


