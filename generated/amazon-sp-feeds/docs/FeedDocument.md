# FeedDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**feed_document_id** | **String** | The identifier for the feed document. This identifier is unique only in combination with a seller ID. | 
**url** | **String** | A presigned URL for the feed document. This URL expires after 5 minutes. | 
**encryption_details** | [**crate::models::FeedDocumentEncryptionDetails**](FeedDocumentEncryptionDetails.md) |  | 
**compression_algorithm** | Option<**String**> | If present, the feed document contents are compressed using the indicated algorithm. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

