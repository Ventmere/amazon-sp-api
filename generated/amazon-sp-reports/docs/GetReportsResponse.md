# GetReportsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payload** | Option<[**Vec<crate::models::Report>**](Report.md)> |  | [optional]
**next_token** | Option<**String**> | Returned when the number of results exceeds pageSize. To get the next page of results, call getReports with this token as the only parameter. | [optional]
**errors** | Option<[**Vec<crate::models::Error>**](Error.md)> | A list of error responses returned when a request is unsuccessful. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


