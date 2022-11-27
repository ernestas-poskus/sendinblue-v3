# \FoldersApi

All URIs are relative to *https://api.sendinblue.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_folder**](FoldersApi.md#create_folder) | **POST** /contacts/folders | Create a folder
[**delete_folder**](FoldersApi.md#delete_folder) | **DELETE** /contacts/folders/{folderId} | Delete a folder (and all its lists)
[**get_folder**](FoldersApi.md#get_folder) | **GET** /contacts/folders/{folderId} | Returns a folder's details
[**get_folder_lists**](FoldersApi.md#get_folder_lists) | **GET** /contacts/folders/{folderId}/lists | Get lists in a folder
[**get_folders**](FoldersApi.md#get_folders) | **GET** /contacts/folders | Get all folders
[**update_folder**](FoldersApi.md#update_folder) | **PUT** /contacts/folders/{folderId} | Update a folder



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

