# ItemOffersRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uri** | **String** | The `getItemOffers` resource path without any query parameters.  **Example:** `/products/pricing/v0/items/B000P6Q7MY/offers` | 
**method** | [**crate::models::HttpMethod**](HttpMethod.md) |  | 
**headers** | Option<**::std::collections::HashMap<String, String>**> | A mapping of additional HTTP headers to send/receive for the individual batch request. | [optional]
**marketplace_id** | **String** | A marketplace identifier. Specifies the marketplace for which prices are returned. | 
**item_condition** | [**crate::models::ItemCondition**](ItemCondition.md) |  | 
**customer_type** | Option<[**crate::models::CustomerType**](CustomerType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


