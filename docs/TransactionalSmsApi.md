# \TransactionalSmsApi

All URIs are relative to *https://api.sendinblue.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sms_events**](TransactionalSmsApi.md#get_sms_events) | **GET** /transactionalSMS/statistics/events | Get all your SMS activity (unaggregated events)
[**get_transac_aggregated_sms_report**](TransactionalSmsApi.md#get_transac_aggregated_sms_report) | **GET** /transactionalSMS/statistics/aggregatedReport | Get your SMS activity aggregated over a period of time
[**get_transac_sms_report**](TransactionalSmsApi.md#get_transac_sms_report) | **GET** /transactionalSMS/statistics/reports | Get your SMS activity aggregated per day
[**send_transac_sms**](TransactionalSmsApi.md#send_transac_sms) | **POST** /transactionalSMS/sms | Send SMS message to a mobile number



## get_sms_events

> crate::models::GetSmsEventReport get_sms_events(limit, start_date, end_date, offset, days, phone_number, event, tags, sort)
Get all your SMS activity (unaggregated events)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of documents per page |  |[default to 50]
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting date (YYYY-MM-DD) of the report |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending date (YYYY-MM-DD) of the report |  |
**offset** | Option<**i64**> | Index of the first document of the page |  |[default to 0]
**days** | Option<**i32**> | Number of days in the past including today (positive integer). Not compatible with 'startDate' and 'endDate' |  |
**phone_number** | Option<**String**> | Filter the report for a specific phone number |  |
**event** | Option<**String**> | Filter the report for specific events |  |
**tags** | Option<**String**> | Filter the report for specific tags passed as a serialized urlencoded array |  |
**sort** | Option<**String**> | Sort the results in the ascending/descending order of record creation |  |[default to desc]

### Return type

[**crate::models::GetSmsEventReport**](getSmsEventReport.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transac_aggregated_sms_report

> crate::models::GetTransacAggregatedSmsReport get_transac_aggregated_sms_report(start_date, end_date, days, tag)
Get your SMS activity aggregated over a period of time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting date (YYYY-MM-DD) of the report |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending date (YYYY-MM-DD) of the report |  |
**days** | Option<**i32**> | Number of days in the past including today (positive integer). Not compatible with startDate and endDate |  |
**tag** | Option<**String**> | Filter on a tag |  |

### Return type

[**crate::models::GetTransacAggregatedSmsReport**](getTransacAggregatedSmsReport.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transac_sms_report

> crate::models::GetTransacSmsReport get_transac_sms_report(start_date, end_date, days, tag, sort)
Get your SMS activity aggregated per day

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting date (YYYY-MM-DD) of the report |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending date (YYYY-MM-DD) of the report |  |
**days** | Option<**i32**> | Number of days in the past including today (positive integer). Not compatible with 'startDate' and 'endDate' |  |
**tag** | Option<**String**> | Filter on a tag |  |
**sort** | Option<**String**> | Sort the results in the ascending/descending order of record creation |  |[default to desc]

### Return type

[**crate::models::GetTransacSmsReport**](getTransacSmsReport.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_transac_sms

> crate::models::SendSms send_transac_sms(send_transac_sms)
Send SMS message to a mobile number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_transac_sms** | [**SendTransacSms**](SendTransacSms.md) | Values to send a transactional SMS | [required] |

### Return type

[**crate::models::SendSms**](sendSms.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

