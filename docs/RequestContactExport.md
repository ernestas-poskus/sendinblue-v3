# RequestContactExport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**export_attributes** | Option<**Vec<String>**> | List of all the attributes that you want to export. These attributes must be present in your contact database. For example, ['fname', 'lname', 'email']. | [optional]
**contact_filter** | Option<[**serde_json::Value**](.md)> | This attribute has been deprecated and will be removed by January 1st, 2021. Only one of the two filter options (contactFilter or customContactFilter) can be passed in the request. Set the filter for the contacts to be exported. For example, {\"blacklisted\":true} will export all the blacklisted contacts.  | [optional]
**custom_contact_filter** | Option<[**crate::models::RequestContactExportCustomContactFilter**](requestContactExport_customContactFilter.md)> |  | [optional]
**notify_url** | Option<**String**> | Webhook that will be called once the export process is finished. For reference, https://help.sendinblue.com/hc/en-us/articles/360007666479 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


