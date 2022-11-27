# \SendersApi

All URIs are relative to *https://api.sendinblue.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sender**](SendersApi.md#create_sender) | **POST** /senders | Create a new sender
[**delete_sender**](SendersApi.md#delete_sender) | **DELETE** /senders/{senderId} | Delete a sender
[**get_ips**](SendersApi.md#get_ips) | **GET** /senders/ips | Get all the dedicated IPs for your account
[**get_ips_from_sender**](SendersApi.md#get_ips_from_sender) | **GET** /senders/{senderId}/ips | Get all the dedicated IPs for a sender
[**get_senders**](SendersApi.md#get_senders) | **GET** /senders | Get the list of all your senders
[**update_sender**](SendersApi.md#update_sender) | **PUT** /senders/{senderId} | Update a sender



## create_sender

> crate::models::CreateSenderModel create_sender(sender)
Create a new sender

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender** | Option<[**CreateSender**](CreateSender.md)> | sender's name |  |

### Return type

[**crate::models::CreateSenderModel**](createSenderModel.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sender

> delete_sender(sender_id)
Delete a sender

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender_id** | **i64** | Id of the sender | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ips

> crate::models::GetIps get_ips()
Get all the dedicated IPs for your account

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetIps**](getIps.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ips_from_sender

> crate::models::GetIpsFromSender get_ips_from_sender(sender_id)
Get all the dedicated IPs for a sender

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender_id** | **i64** | Id of the sender | [required] |

### Return type

[**crate::models::GetIpsFromSender**](getIpsFromSender.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_senders

> crate::models::GetSendersList get_senders(ip, domain)
Get the list of all your senders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip** | Option<**String**> | Filter your senders for a specific ip (available for dedicated IP usage only) |  |
**domain** | Option<**String**> | Filter your senders for a specific domain |  |

### Return type

[**crate::models::GetSendersList**](getSendersList.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sender

> update_sender(sender_id, sender)
Update a sender

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender_id** | **i64** | Id of the sender | [required] |
**sender** | Option<[**UpdateSender**](UpdateSender.md)> | sender's name |  |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

