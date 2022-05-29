# OrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ASIN** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | 
**seller_sku** | Option<**String**> | The seller stock keeping unit (SKU) of the item. | [optional]
**order_item_id** | **String** | An Amazon-defined order item identifier. | 
**title** | Option<**String**> | The name of the item. | [optional]
**quantity_ordered** | **i32** | The number of items in the order.  | 
**quantity_shipped** | Option<**i32**> | The number of items shipped. | [optional]
**product_info** | Option<[**crate::models::ProductInfoDetail**](ProductInfoDetail.md)> |  | [optional]
**points_granted** | Option<[**crate::models::PointsGrantedDetail**](PointsGrantedDetail.md)> |  | [optional]
**item_price** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**shipping_price** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**item_tax** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**shipping_tax** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**shipping_discount** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**shipping_discount_tax** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**promotion_discount** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**promotion_discount_tax** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**promotion_ids** | Option<**Vec<String>**> | A list of promotion identifiers provided by the seller when the promotions were created. | [optional]
**cod_fee** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**cod_fee_discount** | Option<[**crate::models::Money**](Money.md)> |  | [optional]
**is_gift** | Option<**bool**> | When true, the item is a gift. | [optional]
**condition_note** | Option<**String**> | The condition of the item as described by the seller. | [optional]
**condition_id** | Option<**String**> | The condition of the item.  Possible values: New, Used, Collectible, Refurbished, Preorder, Club. | [optional]
**condition_subtype_id** | Option<**String**> | The subcondition of the item.  Possible values: New, Mint, Very Good, Good, Acceptable, Poor, Club, OEM, Warranty, Refurbished Warranty, Refurbished, Open Box, Any, Other. | [optional]
**scheduled_delivery_start_date** | Option<**String**> | The start date of the scheduled delivery window in the time zone of the order destination. In ISO 8601 date time format. | [optional]
**scheduled_delivery_end_date** | Option<**String**> | The end date of the scheduled delivery window in the time zone of the order destination. In ISO 8601 date time format. | [optional]
**price_designation** | Option<**String**> | Indicates that the selling price is a special price that is available only for Amazon Business orders. For more information about the Amazon Business Seller Program, see the [Amazon Business website](https://www.amazon.com/b2b/info/amazon-business).   Possible values: BusinessPrice - A special price that is available only for Amazon Business orders. | [optional]
**tax_collection** | Option<[**crate::models::TaxCollection**](TaxCollection.md)> |  | [optional]
**serial_number_required** | Option<**bool**> | When true, the product type for this item has a serial number.  Returned only for Amazon Easy Ship orders. | [optional]
**is_transparency** | Option<**bool**> | When true, transparency codes are required. | [optional]
**ioss_number** | Option<**String**> | The IOSS number for the marketplace. Sellers shipping to the European Union (EU) from outside of the EU must provide this IOSS number to their carrier when Amazon has collected the VAT on the sale. | [optional]
**store_chain_store_id** | Option<**String**> | The store chain store identifier. Linked to a specific store in a store chain. | [optional]
**deemed_reseller_category** | Option<**String**> | The category of deemed reseller. This applies to selling partners that are not based in the EU and is used to help them meet the VAT Deemed Reseller tax laws in the EU and UK. | [optional]
**buyer_info** | Option<[**crate::models::ItemBuyerInfo**](ItemBuyerInfo.md)> |  | [optional]
**buyer_requested_cancel** | Option<[**crate::models::BuyerRequestedCancel**](BuyerRequestedCancel.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


