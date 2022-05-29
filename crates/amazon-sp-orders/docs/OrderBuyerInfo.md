# OrderBuyerInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon-defined order identifier, in 3-7-7 format. | 
**buyer_email** | Option<**String**> | The anonymized email address of the buyer. | [optional]
**buyer_name** | Option<**String**> | The name of the buyer. | [optional]
**buyer_county** | Option<**String**> | The county of the buyer. | [optional]
**buyer_tax_info** | Option<[**crate::models::BuyerTaxInfo**](BuyerTaxInfo.md)> |  | [optional]
**purchase_order_number** | Option<**String**> | The purchase order (PO) number entered by the buyer at checkout. Returned only for orders where the buyer entered a PO number at checkout. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


