# CreateDoiContact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Email address where the confirmation email will be sent. This email address will be the identifier for all other contact attributes. | 
**attributes** | Option<[**serde_json::Value**](.md)> | Pass the set of attributes and their values. These attributes must be present in your SendinBlue account. For eg. {'FNAME':'Elly', 'LNAME':'Roger'} | [optional]
**include_list_ids** | **Vec<i64>** | Lists under user account where contact should be added | 
**exclude_list_ids** | Option<**Vec<i64>**> | Lists under user account where contact should not be added | [optional]
**template_id** | **i64** | Id of the Double opt-in (DOI) template | 
**redirection_url** | **String** | URL of the web page that user will be redirected to after clicking on the double opt in URL. When editing your DOI template you can reference this URL by using the tag {{ params.DOIurl }}. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


