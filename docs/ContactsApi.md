# \ContactsApi

All URIs are relative to *https://api.sendinblue.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_contact_to_list**](ContactsApi.md#add_contact_to_list) | **POST** /contacts/lists/{listId}/contacts/add | Add existing contacts to a list
[**create_attribute**](ContactsApi.md#create_attribute) | **POST** /contacts/attributes/{attributeCategory}/{attributeName} | Create contact attribute
[**create_contact**](ContactsApi.md#create_contact) | **POST** /contacts | Create a contact
[**create_doi_contact**](ContactsApi.md#create_doi_contact) | **POST** /contacts/doubleOptinConfirmation | Create Contact via DOI (Double-Opt-In) Flow
[**create_folder**](ContactsApi.md#create_folder) | **POST** /contacts/folders | Create a folder
[**create_list**](ContactsApi.md#create_list) | **POST** /contacts/lists | Create a list
[**delete_attribute**](ContactsApi.md#delete_attribute) | **DELETE** /contacts/attributes/{attributeCategory}/{attributeName} | Delete an attribute
[**delete_contact**](ContactsApi.md#delete_contact) | **DELETE** /contacts/{email} | Delete a contact
[**delete_folder**](ContactsApi.md#delete_folder) | **DELETE** /contacts/folders/{folderId} | Delete a folder (and all its lists)
[**delete_list**](ContactsApi.md#delete_list) | **DELETE** /contacts/lists/{listId} | Delete a list
[**get_attributes**](ContactsApi.md#get_attributes) | **GET** /contacts/attributes | List all attributes
[**get_contact_info**](ContactsApi.md#get_contact_info) | **GET** /contacts/{email} | Get a contact's details
[**get_contact_stats**](ContactsApi.md#get_contact_stats) | **GET** /contacts/{email}/campaignStats | Get email campaigns' statistics for a contact
[**get_contacts**](ContactsApi.md#get_contacts) | **GET** /contacts | Get all the contacts
[**get_contacts_from_list**](ContactsApi.md#get_contacts_from_list) | **GET** /contacts/lists/{listId}/contacts | Get contacts in a list
[**get_folder**](ContactsApi.md#get_folder) | **GET** /contacts/folders/{folderId} | Returns a folder's details
[**get_folder_lists**](ContactsApi.md#get_folder_lists) | **GET** /contacts/folders/{folderId}/lists | Get lists in a folder
[**get_folders**](ContactsApi.md#get_folders) | **GET** /contacts/folders | Get all folders
[**get_list**](ContactsApi.md#get_list) | **GET** /contacts/lists/{listId} | Get a list's details
[**get_lists**](ContactsApi.md#get_lists) | **GET** /contacts/lists | Get all the lists
[**import_contacts**](ContactsApi.md#import_contacts) | **POST** /contacts/import | Import contacts
[**remove_contact_from_list**](ContactsApi.md#remove_contact_from_list) | **POST** /contacts/lists/{listId}/contacts/remove | Delete a contact from a list
[**request_contact_export**](ContactsApi.md#request_contact_export) | **POST** /contacts/export | Export contacts
[**update_attribute**](ContactsApi.md#update_attribute) | **PUT** /contacts/attributes/{attributeCategory}/{attributeName} | Update contact attribute
[**update_contact**](ContactsApi.md#update_contact) | **PUT** /contacts/{email} | Update a contact
[**update_folder**](ContactsApi.md#update_folder) | **PUT** /contacts/folders/{folderId} | Update a folder
[**update_list**](ContactsApi.md#update_list) | **PUT** /contacts/lists/{listId} | Update a list



## add_contact_to_list

> crate::models::PostContactInfo add_contact_to_list(list_id, contact_emails)
Add existing contacts to a list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **i64** | Id of the list | [required] |
**contact_emails** | [**AddContactToList**](AddContactToList.md) | Emails addresses of the contacts | [required] |

### Return type

[**crate::models::PostContactInfo**](postContactInfo.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_attribute

> create_attribute(attribute_category, attribute_name, create_attribute)
Create contact attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_category** | **String** | Category of the attribute | [required] |
**attribute_name** | **String** | Name of the attribute | [required] |
**create_attribute** | [**CreateAttribute**](CreateAttribute.md) | Values to create an attribute | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_contact

> crate::models::CreateUpdateContactModel create_contact(create_contact)
Create a contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_contact** | [**CreateContact**](CreateContact.md) | Values to create a contact | [required] |

### Return type

[**crate::models::CreateUpdateContactModel**](createUpdateContactModel.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_doi_contact

> create_doi_contact(create_doi_contact)
Create Contact via DOI (Double-Opt-In) Flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_doi_contact** | [**CreateDoiContact**](CreateDoiContact.md) | Values to create the Double opt-in (DOI) contact | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_folder

> crate::models::CreateModel create_folder(create_folder)
Create a folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_folder** | [**CreateUpdateFolder**](CreateUpdateFolder.md) | Name of the folder | [required] |

### Return type

[**crate::models::CreateModel**](createModel.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_list

> crate::models::CreateModel create_list(create_list)
Create a list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_list** | [**CreateList**](CreateList.md) | Values to create a list | [required] |

### Return type

[**crate::models::CreateModel**](createModel.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_attribute

> delete_attribute(attribute_category, attribute_name)
Delete an attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_category** | **String** | Category of the attribute | [required] |
**attribute_name** | **String** | Name of the existing attribute | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contact

> delete_contact(email)
Delete a contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email (urlencoded) of the contact | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_folder

> delete_folder(folder_id)
Delete a folder (and all its lists)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **i64** | Id of the folder | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_list

> delete_list(list_id)
Delete a list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **i64** | Id of the list | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attributes

> crate::models::GetAttributes get_attributes()
List all attributes

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAttributes**](getAttributes.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contact_info

> crate::models::GetExtendedContactDetails get_contact_info(email)
Get a contact's details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email (urlencoded) of the contact OR its SMS attribute value | [required] |

### Return type

[**crate::models::GetExtendedContactDetails**](getExtendedContactDetails.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contact_stats

> crate::models::GetContactCampaignStats get_contact_stats(email, start_date, end_date)
Get email campaigns' statistics for a contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address (urlencoded) of the contact | [required] |
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be lower than equal to endDate |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be greater than equal to startDate |  |

### Return type

[**crate::models::GetContactCampaignStats**](getContactCampaignStats.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contacts

> crate::models::GetContacts get_contacts(limit, offset, modified_since, sort)
Get all the contacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of documents per page |  |[default to 50]
**offset** | Option<**i64**> | Index of the first document of the page |  |[default to 0]
**modified_since** | Option<**String**> | Filter (urlencoded) the contacts modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result. |  |
**sort** | Option<**String**> | Sort the results in the ascending/descending order of record creation |  |[default to desc]

### Return type

[**crate::models::GetContacts**](getContacts.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contacts_from_list

> crate::models::GetContacts get_contacts_from_list(list_id, modified_since, limit, offset, sort)
Get contacts in a list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **i64** | Id of the list | [required] |
**modified_since** | Option<**String**> | Filter (urlencoded) the contacts modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result. |  |
**limit** | Option<**i64**> | Number of documents per page |  |[default to 50]
**offset** | Option<**i64**> | Index of the first document of the page |  |[default to 0]
**sort** | Option<**String**> | Sort the results in the ascending/descending order of record creation |  |[default to desc]

### Return type

[**crate::models::GetContacts**](getContacts.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder

> crate::models::GetFolder get_folder(folder_id)
Returns a folder's details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **i64** | id of the folder | [required] |

### Return type

[**crate::models::GetFolder**](getFolder.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder_lists

> crate::models::GetFolderLists get_folder_lists(folder_id, limit, offset, sort)
Get lists in a folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **i64** | Id of the folder | [required] |
**limit** | Option<**i64**> | Number of documents per page |  |[default to 10]
**offset** | Option<**i64**> | Index of the first document of the page |  |[default to 0]
**sort** | Option<**String**> | Sort the results in the ascending/descending order of record creation |  |[default to desc]

### Return type

[**crate::models::GetFolderLists**](getFolderLists.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folders

> crate::models::GetFolders get_folders(limit, offset, sort)
Get all folders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i64** | Number of documents per page | [required] |[default to 10]
**offset** | **i64** | Index of the first document of the page | [required] |[default to 0]
**sort** | Option<**String**> | Sort the results in the ascending/descending order of record creation |  |[default to desc]

### Return type

[**crate::models::GetFolders**](getFolders.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list

> crate::models::GetExtendedList get_list(list_id)
Get a list's details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **i64** | Id of the list | [required] |

### Return type

[**crate::models::GetExtendedList**](getExtendedList.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lists

> crate::models::GetLists get_lists(limit, offset, sort)
Get all the lists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of documents per page |  |[default to 10]
**offset** | Option<**i64**> | Index of the first document of the page |  |[default to 0]
**sort** | Option<**String**> | Sort the results in the ascending/descending order of record creation |  |[default to desc]

### Return type

[**crate::models::GetLists**](getLists.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_contacts

> crate::models::CreatedProcessId import_contacts(request_contact_import)
Import contacts

It returns the background process ID which on completion calls the notify URL that you have set in the input.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_contact_import** | [**RequestContactImport**](RequestContactImport.md) | Values to import contacts in Sendinblue. To know more about the expected format, please have a look at ``https://help.sendinblue.com/hc/en-us/articles/209499265-Build-contacts-lists-for-your-email-marketing-campaigns`` | [required] |

### Return type

[**crate::models::CreatedProcessId**](createdProcessId.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_contact_from_list

> crate::models::PostContactInfo remove_contact_from_list(list_id, contact_emails)
Delete a contact from a list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **i64** | Id of the list | [required] |
**contact_emails** | [**RemoveContactFromList**](RemoveContactFromList.md) | Emails adresses of the contact | [required] |

### Return type

[**crate::models::PostContactInfo**](postContactInfo.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_contact_export

> crate::models::CreatedProcessId request_contact_export(request_contact_export)
Export contacts

It returns the background process ID which on completion calls the notify URL that you have set in the input. File will be available in csv.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_contact_export** | [**RequestContactExport**](RequestContactExport.md) | Values to request a contact export | [required] |

### Return type

[**crate::models::CreatedProcessId**](createdProcessId.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_attribute

> update_attribute(attribute_category, attribute_name, update_attribute)
Update contact attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_category** | **String** | Category of the attribute | [required] |
**attribute_name** | **String** | Name of the existing attribute | [required] |
**update_attribute** | [**UpdateAttribute**](UpdateAttribute.md) | Values to update an attribute | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_contact

> update_contact(email, update_contact)
Update a contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email (urlencoded) of the contact | [required] |
**update_contact** | [**UpdateContact**](UpdateContact.md) | Values to update a contact | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_folder

> update_folder(folder_id, update_folder)
Update a folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **i64** | Id of the folder | [required] |
**update_folder** | [**CreateUpdateFolder**](CreateUpdateFolder.md) | Name of the folder | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_list

> update_list(list_id, update_list)
Update a list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **i64** | Id of the list | [required] |
**update_list** | [**UpdateList**](UpdateList.md) | Values to update a list | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

