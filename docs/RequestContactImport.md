# RequestContactImport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_url** | Option<**String**> | Mandatory if fileBody is not defined. URL of the file to be imported (no local file). Possible file formats: .txt, .csv | [optional]
**file_body** | Option<**String**> | Mandatory if fileUrl is not defined. CSV content to be imported. Use semicolon to separate multiple attributes. Maximum allowed file body size is 10MB . However we recommend a safe limit of around 8 MB to avoid the issues caused due to increase of file body size while parsing. Please use fileUrl instead to import bigger files. | [optional]
**list_ids** | Option<**Vec<i64>**> | Mandatory if newList is not defined. Ids of the lists in which the contacts shall be imported. For example, [2, 4, 7]. | [optional]
**notify_url** | Option<**String**> | URL that will be called once the import process is finished. For reference, https://help.sendinblue.com/hc/en-us/articles/360007666479 | [optional]
**new_list** | Option<[**crate::models::RequestContactImportNewList**](requestContactImport_newList.md)> |  | [optional]
**email_blacklist** | Option<**bool**> | To blacklist all the contacts for email | [optional][default to false]
**sms_blacklist** | Option<**bool**> | To blacklist all the contacts for sms | [optional][default to false]
**update_existing_contacts** | Option<**bool**> | To facilitate the choice to update the existing contacts | [optional][default to true]
**empty_contacts_attributes** | Option<**bool**> | To facilitate the choice to erase any attribute of the existing contacts with empty value. emptyContactsAttributes = true means the empty fields in your import will erase any attribute that currently contain data in SendinBlue, & emptyContactsAttributes = false means the empty fields will not affect your existing data ( only available if `updateExistingContacts` set to true ) | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


