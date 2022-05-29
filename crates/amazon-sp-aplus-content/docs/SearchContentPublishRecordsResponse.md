# SearchContentPublishRecordsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**warnings** | Option<[**Vec<crate::models::Error>**](Error.md)> | A set of messages to the user, such as warnings or comments. | [optional]
**next_page_token** | Option<**String**> | A page token that is returned when the results of the call exceed the page size. To get another page of results, call the operation again, passing in this value with the pageToken parameter. | [optional]
**publish_record_list** | [**Vec<crate::models::PublishRecord>**](PublishRecord.md) | A list of A+ Content publishing records. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


