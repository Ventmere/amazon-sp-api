# CreateFeedSpecification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**feed_type** | **String** | The feed type. | 
**marketplace_ids** | **Vec<String>** | A list of identifiers for marketplaces that you want the feed to be applied to. | 
**input_feed_document_id** | **String** | The document identifier returned by the createFeedDocument operation. Encrypt and upload the feed document contents before calling the createFeed operation. | 
**feed_options** | Option<**::std::collections::HashMap<String, String>**> | Additional options to control the feed. For feeds that use the feedOptions parameter, you can find the parameter values in the feed description in [feedType values](https://github.com/amzn/selling-partner-api-docs/blob/main/references/feeds-api/feedtype-values.md). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


