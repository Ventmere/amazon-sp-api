# UpdateVerificationStatusRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**crate::models::VerificationStatus**](VerificationStatus.md) |  | 
**external_reviewer_id** | **String** | The identifier for the order's regulated information reviewer. | 
**rejection_reason_id** | Option<**String**> | The unique identifier for the rejection reason used for rejecting the order's regulated information. Only required if the new status is rejected. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


