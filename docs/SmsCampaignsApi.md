# \SmsCampaignsApi

All URIs are relative to *https://api.sendinblue.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sms_campaign**](SmsCampaignsApi.md#create_sms_campaign) | **POST** /smsCampaigns | Creates an SMS campaign
[**delete_sms_campaign**](SmsCampaignsApi.md#delete_sms_campaign) | **DELETE** /smsCampaigns/{campaignId} | Delete an SMS campaign
[**get_sms_campaign**](SmsCampaignsApi.md#get_sms_campaign) | **GET** /smsCampaigns/{campaignId} | Get an SMS campaign
[**get_sms_campaigns**](SmsCampaignsApi.md#get_sms_campaigns) | **GET** /smsCampaigns | Returns the information for all your created SMS campaigns
[**request_sms_recipient_export**](SmsCampaignsApi.md#request_sms_recipient_export) | **POST** /smsCampaigns/{campaignId}/exportRecipients | Export an SMS campaign's recipients
[**send_sms_campaign_now**](SmsCampaignsApi.md#send_sms_campaign_now) | **POST** /smsCampaigns/{campaignId}/sendNow | Send your SMS campaign immediately
[**send_sms_report**](SmsCampaignsApi.md#send_sms_report) | **POST** /smsCampaigns/{campaignId}/sendReport | Send an SMS campaign's report
[**send_test_sms**](SmsCampaignsApi.md#send_test_sms) | **POST** /smsCampaigns/{campaignId}/sendTest | Send a test SMS campaign
[**update_sms_campaign**](SmsCampaignsApi.md#update_sms_campaign) | **PUT** /smsCampaigns/{campaignId} | Update an SMS campaign
[**update_sms_campaign_status**](SmsCampaignsApi.md#update_sms_campaign_status) | **PUT** /smsCampaigns/{campaignId}/status | Update a campaign's status



## create_sms_campaign

> crate::models::CreateModel create_sms_campaign(create_sms_campaign)
Creates an SMS campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_sms_campaign** | [**CreateSmsCampaign**](CreateSmsCampaign.md) | Values to create an SMS Campaign | [required] |

### Return type

[**crate::models::CreateModel**](createModel.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sms_campaign

> delete_sms_campaign(campaign_id)
Delete an SMS campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | id of the SMS campaign | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sms_campaign

> crate::models::GetSmsCampaign get_sms_campaign(campaign_id)
Get an SMS campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | id of the SMS campaign | [required] |

### Return type

[**crate::models::GetSmsCampaign**](getSmsCampaign.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sms_campaigns

> crate::models::GetSmsCampaigns get_sms_campaigns(status, start_date, end_date, limit, offset)
Returns the information for all your created SMS campaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Status of campaign. |  |
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent sms campaigns. Prefer to pass your timezone in date-time format for accurate result ( only available if either 'status' not passed and if passed is set to 'sent' ) |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent sms campaigns. Prefer to pass your timezone in date-time format for accurate result ( only available if either 'status' not passed and if passed is set to 'sent' ) |  |
**limit** | Option<**i64**> | Number limitation for the result returned |  |[default to 500]
**offset** | Option<**i64**> | Beginning point in the list to retrieve from. |  |[default to 0]

### Return type

[**crate::models::GetSmsCampaigns**](getSmsCampaigns.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_sms_recipient_export

> crate::models::CreatedProcessId request_sms_recipient_export(campaign_id, recipient_export)
Export an SMS campaign's recipients

It returns the background process ID which on completion calls the notify URL that you have set in the input.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | id of the campaign | [required] |
**recipient_export** | Option<[**RequestSmsRecipientExport**](RequestSmsRecipientExport.md)> | Values to send for a recipient export request |  |

### Return type

[**crate::models::CreatedProcessId**](createdProcessId.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_sms_campaign_now

> send_sms_campaign_now(campaign_id)
Send your SMS campaign immediately

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | id of the campaign | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_sms_report

> send_sms_report(campaign_id, send_report)
Send an SMS campaign's report

Send report of Sent and Archived campaign, to the specified email addresses, with respective data and a pdf attachment in detail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | id of the campaign | [required] |
**send_report** | [**SendReport**](SendReport.md) | Values for send a report | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_test_sms

> send_test_sms(campaign_id, phone_number)
Send a test SMS campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | Id of the SMS campaign | [required] |
**phone_number** | [**SendTestSms**](SendTestSms.md) | Mobile number of the recipient with the country code. This number must belong to one of your contacts in SendinBlue account and must not be blacklisted | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sms_campaign

> update_sms_campaign(campaign_id, update_sms_campaign)
Update an SMS campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | id of the SMS campaign | [required] |
**update_sms_campaign** | [**UpdateSmsCampaign**](UpdateSmsCampaign.md) | Values to update an SMS Campaign | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sms_campaign_status

> update_sms_campaign_status(campaign_id, status)
Update a campaign's status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | id of the campaign | [required] |
**status** | [**UpdateCampaignStatus**](UpdateCampaignStatus.md) | Status of the campaign. | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

