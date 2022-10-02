# ReportDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**report_document_id** | **String** | The identifier for the report document. This identifier is unique only in combination with a seller ID. | 
**url** | **String** | A presigned URL for the report document. This URL expires after 5 minutes. | 
**encryption_details** | [**crate::models::ReportDocumentEncryptionDetails**](ReportDocumentEncryptionDetails.md) |  | 
**compression_algorithm** | Option<**String**> | If present, the report document contents have been compressed with the provided algorithm. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


