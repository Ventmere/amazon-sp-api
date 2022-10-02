# Summary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_offer_count** | **i32** | The number of unique offers contained in NumberOfOffers. | 
**number_of_offers** | Option<[**Vec<crate::models::OfferCountType>**](OfferCountType.md)> |  | [optional]
**lowest_prices** | Option<[**Vec<crate::models::LowestPriceType>**](LowestPriceType.md)> |  | [optional]
**buy_box_prices** | Option<[**Vec<crate::models::BuyBoxPriceType>**](BuyBoxPriceType.md)> |  | [optional]
**list_price** | Option<[**crate::models::MoneyType**](MoneyType.md)> |  | [optional]
**competitive_price_threshold** | Option<[**crate::models::MoneyType**](MoneyType.md)> |  | [optional]
**suggested_lower_price_plus_shipping** | Option<[**crate::models::MoneyType**](MoneyType.md)> |  | [optional]
**sales_rankings** | Option<[**Vec<crate::models::SalesRankType>**](SalesRankType.md)> | A list of sales rank information for the item, by category. | [optional]
**buy_box_eligible_offers** | Option<[**Vec<crate::models::OfferCountType>**](OfferCountType.md)> |  | [optional]
**offers_available_time** | Option<**String**> | When the status is ActiveButTooSoonForProcessing, this is the time when the offers will be available for processing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


