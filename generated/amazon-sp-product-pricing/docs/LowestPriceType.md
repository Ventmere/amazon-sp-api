# LowestPriceType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition** | **String** | Indicates the condition of the item. For example: New, Used, Collectible, Refurbished, or Club. | 
**fulfillment_channel** | **String** | Indicates whether the item is fulfilled by Amazon or by the seller. | 
**offer_type** | Option<[**crate::models::OfferCustomerType**](OfferCustomerType.md)> |  | [optional]
**quantity_tier** | Option<**i32**> | Indicates at what quantity this price becomes active. | [optional]
**quantity_discount_type** | Option<[**crate::models::QuantityDiscountType**](QuantityDiscountType.md)> |  | [optional]
**landed_price** | [**crate::models::MoneyType**](MoneyType.md) |  | 
**listing_price** | [**crate::models::MoneyType**](MoneyType.md) |  | 
**shipping** | [**crate::models::MoneyType**](MoneyType.md) |  | 
**points** | Option<[**crate::models::Points**](Points.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


