# AffordabilityExpenseEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | Option<**String**> | An Amazon-defined identifier for an order. | [optional]
**posted_date** | Option<**String**> |  | [optional]
**marketplace_id** | Option<**String**> | An encrypted, Amazon-defined marketplace identifier. | [optional]
**transaction_type** | Option<**String**> | Indicates the type of transaction.   Possible values:  * Charge - For an affordability promotion expense.  * Refund - For an affordability promotion expense reversal. | [optional]
**base_expense** | Option<[**crate::models::Currency**](Currency.md)> |  | [optional]
**tax_type_cgst** | [**crate::models::Currency**](Currency.md) |  | 
**tax_type_sgst** | [**crate::models::Currency**](Currency.md) |  | 
**tax_type_igst** | [**crate::models::Currency**](Currency.md) |  | 
**total_expense** | Option<[**crate::models::Currency**](Currency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


