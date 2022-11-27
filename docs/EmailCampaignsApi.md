# \EmailCampaignsApi

All URIs are relative to *https://api.sendinblue.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_email_campaign**](EmailCampaignsApi.md#create_email_campaign) | **POST** /emailCampaigns | Create an email campaign
[**delete_email_campaign**](EmailCampaignsApi.md#delete_email_campaign) | **DELETE** /emailCampaigns/{campaignId} | Delete an email campaign
[**email_export_recipients**](EmailCampaignsApi.md#email_export_recipients) | **POST** /emailCampaigns/{campaignId}/exportRecipients | Export the recipients of an email campaign
[**get_ab_test_campaign_result**](EmailCampaignsApi.md#get_ab_test_campaign_result) | **GET** /emailCampaigns/{campaignId}/abTestCampaignResult | Get an A/B test email campaign results
[**get_email_campaign**](EmailCampaignsApi.md#get_email_campaign) | **GET** /emailCampaigns/{campaignId} | Get an email campaign report
[**get_email_campaigns**](EmailCampaignsApi.md#get_email_campaigns) | **GET** /emailCampaigns | Return all your created email campaigns
[**get_shared_template_url**](EmailCampaignsApi.md#get_shared_template_url) | **GET** /emailCampaigns/{campaignId}/sharedUrl | Get a shared template url
[**send_email_campaign_now**](EmailCampaignsApi.md#send_email_campaign_now) | **POST** /emailCampaigns/{campaignId}/sendNow | Send an email campaign immediately, based on campaignId
[**send_report**](EmailCampaignsApi.md#send_report) | **POST** /emailCampaigns/{campaignId}/sendReport | Send the report of a campaign
[**send_test_email**](EmailCampaignsApi.md#send_test_email) | **POST** /emailCampaigns/{campaignId}/sendTest | Send an email campaign to your test list
[**update_campaign_status**](EmailCampaignsApi.md#update_campaign_status) | **PUT** /emailCampaigns/{campaignId}/status | Update an email campaign status
[**update_email_campaign**](EmailCampaignsApi.md#update_email_campaign) | **PUT** /emailCampaigns/{campaignId} | Update an email campaign
[**upload_image_to_gallery**](EmailCampaignsApi.md#upload_image_to_gallery) | **POST** /emailCampaigns/images | Upload an image to your account's image gallery



## create_email_campaign

> crate::models::CreateModel create_email_campaign(email_campaigns)
Create an email campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_campaigns** | [**CreateEmailCampaign**](CreateEmailCampaign.md) | Values to create a campaign | [required] |

### Return type

[**crate::models::CreateModel**](createModel.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_email_campaign

> delete_email_campaign(campaign_id)
Delete an email campaign

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


## email_export_recipients

> crate::models::CreatedProcessId email_export_recipients(campaign_id, recipient_export)
Export the recipients of an email campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | Id of the campaign | [required] |
**recipient_export** | Option<[**EmailExportRecipients**](EmailExportRecipients.md)> | Values to send for a recipient export request |  |

### Return type

[**crate::models::CreatedProcessId**](createdProcessId.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ab_test_campaign_result

> crate::models::AbTestCampaignResult get_ab_test_campaign_result(campaign_id)
Get an A/B test email campaign results

Obtain winning version of an A/B test email campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | Id of the A/B test campaign | [required] |

### Return type

[**crate::models::AbTestCampaignResult**](abTestCampaignResult.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_campaign

> crate::models::GetEmailCampaign get_email_campaign(campaign_id)
Get an email campaign report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | Id of the campaign | [required] |

### Return type

[**crate::models::GetEmailCampaign**](getEmailCampaign.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_campaigns

> crate::models::GetEmailCampaigns get_email_campaigns(r#type, status, start_date, end_date, limit, offset)
Return all your created email campaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Filter on the type of the campaigns |  |
**status** | Option<**String**> | Filter on the status of the campaign |  |
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent email campaigns. Prefer to pass your timezone in date-time format for accurate result ( only available if either 'status' not passed and if passed is set to 'sent' ) |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent email campaigns. Prefer to pass your timezone in date-time format for accurate result ( only available if either 'status' not passed and if passed is set to 'sent' ) |  |
**limit** | Option<**i64**> | Number of documents per page |  |[default to 500]
**offset** | Option<**i64**> | Index of the first document in the page |  |[default to 0]

### Return type

[**crate::models::GetEmailCampaigns**](getEmailCampaigns.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, applications/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shared_template_url

> crate::models::GetSharedTemplateUrl get_shared_template_url(campaign_id)
Get a shared template url

Get a unique URL to share & import an email template from one Sendinblue account to another.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | Id of the campaign or template | [required] |

### Return type

[**crate::models::GetSharedTemplateUrl**](getSharedTemplateUrl.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_email_campaign_now

> send_email_campaign_now(campaign_id)
Send an email campaign immediately, based on campaignId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | Id of the campaign | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_report

> send_report(campaign_id, send_report)
Send the report of a campaign

A PDF will be sent to the specified email addresses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | Id of the campaign | [required] |
**send_report** | [**SendReport**](SendReport.md) | Values for send a report | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_test_email

> send_test_email(campaign_id, email_to)
Send an email campaign to your test list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | Id of the campaign | [required] |
**email_to** | [**SendTestEmail**](SendTestEmail.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_campaign_status

> update_campaign_status(campaign_id, status)
Update an email campaign status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | Id of the campaign | [required] |
**status** | [**UpdateCampaignStatus**](UpdateCampaignStatus.md) | Status of the campaign | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_email_campaign

> update_email_campaign(campaign_id, email_campaign)
Update an email campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **i64** | Id of the campaign | [required] |
**email_campaign** | [**UpdateEmailCampaign**](UpdateEmailCampaign.md) | Values to update a campaign | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_image_to_gallery

> upload_image_to_gallery(upload_image)
Upload an image to your account's image gallery

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_image** | [**UploadImageToGallery**](UploadImageToGallery.md) | Parameters to upload an image | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

