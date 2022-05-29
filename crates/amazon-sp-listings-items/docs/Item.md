# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sku** | **String** | A selling partner provided identifier for an Amazon listing. | 
**summaries** | Option<[**Vec<crate::models::ItemSummaryByMarketplace>**](ItemSummaryByMarketplace.md)> | Summary details of a listings item. | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> | JSON object containing structured listings item attribute data keyed by attribute name. | [optional]
**issues** | Option<[**Vec<crate::models::Issue>**](Issue.md)> | Issues associated with the listings item. | [optional]
**offers** | Option<[**Vec<crate::models::ItemOfferByMarketplace>**](ItemOfferByMarketplace.md)> | Offer details for the listings item. | [optional]
**fulfillment_availability** | Option<[**Vec<crate::models::FulfillmentAvailability>**](FulfillmentAvailability.md)> | Fulfillment availability for the listings item. | [optional]
**procurement** | Option<[**crate::models::ItemProcurement**](ItemProcurement.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


