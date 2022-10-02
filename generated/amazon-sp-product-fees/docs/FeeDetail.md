# FeeDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fee_type** | **String** | The type of fee charged to a seller. | 
**fee_amount** | [**crate::models::MoneyType**](MoneyType.md) |  | 
**fee_promotion** | Option<[**crate::models::MoneyType**](MoneyType.md)> |  | [optional]
**tax_amount** | Option<[**crate::models::MoneyType**](MoneyType.md)> |  | [optional]
**final_fee** | [**crate::models::MoneyType**](MoneyType.md) |  | 
**included_fee_detail_list** | Option<[**Vec<crate::models::IncludedFeeDetail>**](IncludedFeeDetail.md)> | A list of other fees that contribute to a given fee. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


