# \ListsApi

All URIs are relative to *https://api.sendinblue.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_contact_to_list**](ListsApi.md#add_contact_to_list) | **POST** /contacts/lists/{listId}/contacts/add | Add existing contacts to a list
[**create_list**](ListsApi.md#create_list) | **POST** /contacts/lists | Create a list
[**delete_list**](ListsApi.md#delete_list) | **DELETE** /contacts/lists/{listId} | Delete a list
[**get_contacts_from_list**](ListsApi.md#get_contacts_from_list) | **GET** /contacts/lists/{listId}/contacts | Get contacts in a list
[**get_folder_lists**](ListsApi.md#get_folder_lists) | **GET** /contacts/folders/{folderId}/lists | Get lists in a folder
[**get_list**](ListsApi.md#get_list) | **GET** /contacts/lists/{listId} | Get a list's details
[**get_lists**](ListsApi.md#get_lists) | **GET** /contacts/lists | Get all the lists
[**remove_contact_from_list**](ListsApi.md#remove_contact_from_list) | **POST** /contacts/lists/{listId}/contacts/remove | Delete a contact from a list
[**update_list**](ListsApi.md#update_list) | **PUT** /contacts/lists/{listId} | Update a list



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

