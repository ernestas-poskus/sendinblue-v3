# RemoveContactFromList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**emails** | Option<**Vec<String>**> | Required if 'all' is false. Emails to remove from a list. You can pass a maximum of 150 emails for removal in one request. | [optional]
**all** | Option<**bool**> | Required if 'emails' is empty. Remove all existing contacts from a list. A process will be created in this scenario. You can fetch the process details to know about the progress | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


