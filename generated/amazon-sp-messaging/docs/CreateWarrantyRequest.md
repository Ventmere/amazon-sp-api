# CreateWarrantyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachments** | Option<[**Vec<crate::models::Attachment>**](Attachment.md)> | Attachments to include in the message to the buyer. If any text is included in the attachment, the text must be written in the buyer's language of preference, which can be retrieved from the GetAttributes operation. | [optional]
**coverage_start_date** | Option<**String**> | The start date of the warranty coverage to include in the message to the buyer. | [optional]
**coverage_end_date** | Option<**String**> | The end date of the warranty coverage to include in the message to the buyer. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


