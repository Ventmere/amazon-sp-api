# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | **String** | Amazon Standard Identification Number (ASIN) is the unique identifier for an item in the Amazon catalog. | 
**attributes** | Option<[**serde_json::Value**](.md)> | A JSON object that contains structured item attribute data keyed by attribute name. Catalog item attributes conform to the related product type definitions available in the Selling Partner API for Product Type Definitions. | [optional]
**dimensions** | Option<[**Vec<crate::models::ItemDimensionsByMarketplace>**](ItemDimensionsByMarketplace.md)> | Array of dimensions associated with the item in the Amazon catalog by Amazon marketplace. | [optional]
**identifiers** | Option<[**Vec<crate::models::ItemIdentifiersByMarketplace>**](ItemIdentifiersByMarketplace.md)> | Identifiers associated with the item in the Amazon catalog, such as UPC and EAN identifiers. | [optional]
**images** | Option<[**Vec<crate::models::ItemImagesByMarketplace>**](ItemImagesByMarketplace.md)> | Images for an item in the Amazon catalog. | [optional]
**product_types** | Option<[**Vec<crate::models::ItemProductTypeByMarketplace>**](ItemProductTypeByMarketplace.md)> | Product types associated with the Amazon catalog item. | [optional]
**relationships** | Option<[**Vec<crate::models::ItemRelationshipsByMarketplace>**](ItemRelationshipsByMarketplace.md)> | Relationships by marketplace for an Amazon catalog item (for example, variations). | [optional]
**sales_ranks** | Option<[**Vec<crate::models::ItemSalesRanksByMarketplace>**](ItemSalesRanksByMarketplace.md)> | Sales ranks of an Amazon catalog item. | [optional]
**summaries** | Option<[**Vec<crate::models::ItemSummaryByMarketplace>**](ItemSummaryByMarketplace.md)> | Summary details of an Amazon catalog item. | [optional]
**vendor_details** | Option<[**Vec<crate::models::ItemVendorDetailsByMarketplace>**](ItemVendorDetailsByMarketplace.md)> | Vendor details associated with an Amazon catalog item. Vendor details are available to vendors only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


