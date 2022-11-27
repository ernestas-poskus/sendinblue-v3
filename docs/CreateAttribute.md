# CreateAttribute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**value** | Option<**String**> | Value of the attribute. Use only if the attribute's category is 'calculated' or 'global' | [optional]
**enumeration** | Option<[**Vec<crate::models::CreateAttributeEnumerationInner>**](createAttribute_enumeration_inner.md)> | List of values and labels that the attribute can take. Use only if the attribute's category is \"category\". For example, [{\"value\":1, \"label\":\"male\"}, {\"value\":2, \"label\":\"female\"}] | [optional]
**r#type** | Option<**String**> | Type of the attribute. Use only if the attribute's category is 'normal', 'category' or 'transactional' ( type 'boolean' is only available if the category is 'normal' attribute, type 'id' is only available if the category is 'transactional' attribute & type 'category' is only available if the category is 'category' attribute ) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


