# ContentDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The A+ Content document name. | 
**content_type** | [**crate::models::ContentType**](ContentType.md) |  | 
**content_sub_type** | Option<**String**> | The A+ Content document subtype. This represents a special-purpose type of an A+ Content document. Not every A+ Content document type will have a subtype, and subtypes may change at any time. | [optional]
**locale** | **String** | The IETF language tag. This only supports the primary language subtag with one secondary language subtag. The secondary language subtag is almost always a regional designation. This does not support additional subtags beyond the primary and secondary subtags. **Pattern:** ^[a-z]{2,}-[A-Z0-9]{2,}$ | 
**content_module_list** | [**Vec<crate::models::ContentModule>**](ContentModule.md) | A list of A+ Content modules. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


