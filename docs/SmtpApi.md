# \SmtpApi

All URIs are relative to *https://api.sendinblue.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**block_new_domain**](SmtpApi.md#block_new_domain) | **POST** /smtp/blockedDomains | Add a new domain to the list of blocked domains
[**create_smtp_template**](SmtpApi.md#create_smtp_template) | **POST** /smtp/templates | Create an email template
[**delete_blocked_domain**](SmtpApi.md#delete_blocked_domain) | **DELETE** /smtp/blockedDomains/{domain} | Unblock an existing domain from the list of blocked domains
[**delete_hardbounces**](SmtpApi.md#delete_hardbounces) | **POST** /smtp/deleteHardbounces | Delete hardbounces
[**delete_smtp_template**](SmtpApi.md#delete_smtp_template) | **DELETE** /smtp/templates/{templateId} | Delete an inactive email template
[**get_aggregated_smtp_report**](SmtpApi.md#get_aggregated_smtp_report) | **GET** /smtp/statistics/aggregatedReport | Get your transactional email activity aggregated over a period of time
[**get_blocked_domains**](SmtpApi.md#get_blocked_domains) | **GET** /smtp/blockedDomains | Get the list of blocked domains
[**get_email_event_report**](SmtpApi.md#get_email_event_report) | **GET** /smtp/statistics/events | Get all your transactional email activity (unaggregated events)
[**get_smtp_report**](SmtpApi.md#get_smtp_report) | **GET** /smtp/statistics/reports | Get your transactional email activity aggregated per day
[**get_smtp_template**](SmtpApi.md#get_smtp_template) | **GET** /smtp/templates/{templateId} | Returns the template information
[**get_smtp_templates**](SmtpApi.md#get_smtp_templates) | **GET** /smtp/templates | Get the list of email templates
[**get_transac_blocked_contacts**](SmtpApi.md#get_transac_blocked_contacts) | **GET** /smtp/blockedContacts | Get the list of blocked or unsubscribed transactional contacts
[**get_transac_email_content**](SmtpApi.md#get_transac_email_content) | **GET** /smtp/emails/{uuid} | Get the personalized content of a sent transactional email
[**get_transac_emails_list**](SmtpApi.md#get_transac_emails_list) | **GET** /smtp/emails | Get the list of transactional emails on the basis of allowed filters
[**send_template**](SmtpApi.md#send_template) | **POST** /smtp/templates/{templateId}/send | Send a template
[**send_test_template**](SmtpApi.md#send_test_template) | **POST** /smtp/templates/{templateId}/sendTest | Send a template to your test list
[**send_transac_email**](SmtpApi.md#send_transac_email) | **POST** /smtp/email | Send a transactional email
[**smtp_blocked_contacts_email_delete**](SmtpApi.md#smtp_blocked_contacts_email_delete) | **DELETE** /smtp/blockedContacts/{email} | Unblock or resubscribe a transactional contact
[**smtp_log_message_id_delete**](SmtpApi.md#smtp_log_message_id_delete) | **DELETE** /smtp/log/{messageId} | Delete an SMTP transactional log
[**update_smtp_template**](SmtpApi.md#update_smtp_template) | **PUT** /smtp/templates/{templateId} | Update an email template



## block_new_domain

> block_new_domain(block_domain)
Add a new domain to the list of blocked domains

Blocks a new domain in order to avoid messages being sent to the same

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_domain** | [**BlockDomain**](BlockDomain.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_smtp_template

> crate::models::CreateModel create_smtp_template(smtp_template)
Create an email template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smtp_template** | [**CreateSmtpTemplate**](CreateSmtpTemplate.md) | values to update in transactional email template | [required] |

### Return type

[**crate::models::CreateModel**](createModel.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_blocked_domain

> delete_blocked_domain(domain)
Unblock an existing domain from the list of blocked domains

Unblocks an existing domain from the list of blocked domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | The name of the domain to be deleted | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_hardbounces

> delete_hardbounces(delete_hardbounces)
Delete hardbounces

Delete hardbounces. To use carefully (e.g. in case of temporary ISP failures)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_hardbounces** | Option<[**DeleteHardbounces**](DeleteHardbounces.md)> | values to delete hardbounces |  |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_smtp_template

> delete_smtp_template(template_id)
Delete an inactive email template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i64** | id of the template | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aggregated_smtp_report

> crate::models::GetAggregatedReport get_aggregated_smtp_report(start_date, end_date, days, tag)
Get your transactional email activity aggregated over a period of time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting date of the report (YYYY-MM-DD). Must be lower than equal to endDate |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending date of the report (YYYY-MM-DD). Must be greater than equal to startDate |  |
**days** | Option<**i32**> | Number of days in the past including today (positive integer). Not compatible with 'startDate' and 'endDate' |  |
**tag** | Option<**String**> | Tag of the emails |  |

### Return type

[**crate::models::GetAggregatedReport**](getAggregatedReport.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blocked_domains

> crate::models::GetBlockedDomains get_blocked_domains()
Get the list of blocked domains

Get the list of blocked domains

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetBlockedDomains**](getBlockedDomains.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_event_report

> crate::models::GetEmailEventReport get_email_event_report(limit, offset, start_date, end_date, days, email, event, tags, message_id, template_id)
Get all your transactional email activity (unaggregated events)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number limitation for the result returned |  |[default to 50]
**offset** | Option<**i64**> | Beginning point in the list to retrieve from. |  |[default to 0]
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting date of the report (YYYY-MM-DD). Must be lower than equal to endDate |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending date of the report (YYYY-MM-DD). Must be greater than equal to startDate |  |
**days** | Option<**i32**> | Number of days in the past including today (positive integer). Not compatible with 'startDate' and 'endDate' |  |
**email** | Option<**String**> | Filter the report for a specific email addresses |  |
**event** | Option<**String**> | Filter the report for a specific event type |  |
**tags** | Option<**String**> | Filter the report for tags (serialized and urlencoded array) |  |
**message_id** | Option<**String**> | Filter on a specific message id |  |
**template_id** | Option<**i64**> | Filter on a specific template id |  |

### Return type

[**crate::models::GetEmailEventReport**](getEmailEventReport.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_smtp_report

> crate::models::GetReports get_smtp_report(limit, offset, start_date, end_date, days, tag)
Get your transactional email activity aggregated per day

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of documents returned per page |  |[default to 10]
**offset** | Option<**i64**> | Index of the first document on the page |  |[default to 0]
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting date of the report (YYYY-MM-DD) |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending date of the report (YYYY-MM-DD) |  |
**days** | Option<**i32**> | Number of days in the past including today (positive integer). Not compatible with 'startDate' and 'endDate' |  |
**tag** | Option<**String**> | Tag of the emails |  |

### Return type

[**crate::models::GetReports**](getReports.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_smtp_template

> crate::models::GetSmtpTemplateOverview get_smtp_template(template_id)
Returns the template information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i64** | id of the template | [required] |

### Return type

[**crate::models::GetSmtpTemplateOverview**](getSmtpTemplateOverview.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_smtp_templates

> crate::models::GetSmtpTemplates get_smtp_templates(template_status, limit, offset)
Get the list of email templates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_status** | Option<**bool**> | Filter on the status of the template. Active = true, inactive = false |  |
**limit** | Option<**i64**> | Number of documents returned per page |  |[default to 50]
**offset** | Option<**i64**> | Index of the first document in the page |  |[default to 0]

### Return type

[**crate::models::GetSmtpTemplates**](getSmtpTemplates.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transac_blocked_contacts

> crate::models::GetTransacBlockedContacts get_transac_blocked_contacts(start_date, end_date, limit, offset, senders)
Get the list of blocked or unsubscribed transactional contacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting date (YYYY-MM-DD) from which you want to fetch the blocked or unsubscribed contacts |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending date (YYYY-MM-DD) till which you want to fetch the blocked or unsubscribed contacts |  |
**limit** | Option<**i64**> | Number of documents returned per page |  |[default to 50]
**offset** | Option<**i64**> | Index of the first document on the page |  |[default to 0]
**senders** | Option<[**Vec<String>**](String.md)> | Comma separated list of emails of the senders from which contacts are blocked or unsubscribed |  |

### Return type

[**crate::models::GetTransacBlockedContacts**](getTransacBlockedContacts.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transac_email_content

> crate::models::GetTransacEmailContent get_transac_email_content(uuid)
Get the personalized content of a sent transactional email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Unique id of the transactional email that has been sent to a particular contact | [required] |

### Return type

[**crate::models::GetTransacEmailContent**](getTransacEmailContent.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transac_emails_list

> crate::models::GetTransacEmailsList get_transac_emails_list(email, template_id, message_id, start_date, end_date)
Get the list of transactional emails on the basis of allowed filters

This endpoint will show the list of emails for past 30 days by default. To retrieve emails before that time, please pass startDate and endDate in query filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<**String**> | Mandatory if templateId and messageId are not passed in query filters. Email address to which transactional email has been sent. |  |
**template_id** | Option<**i64**> | Mandatory if email and messageId are not passed in query filters. Id of the template that was used to compose transactional email. |  |
**message_id** | Option<**String**> | Mandatory if templateId and email are not passed in query filters. Message ID of the transactional email sent. |  |
**start_date** | Option<**String**> | Mandatory if endDate is used. Starting date (YYYY-MM-DD) from which you want to fetch the list. Maximum time period that can be selected is one month. |  |
**end_date** | Option<**String**> | Mandatory if startDate is used. Ending date (YYYY-MM-DD) till which you want to fetch the list. Maximum time period that can be selected is one month. |  |

### Return type

[**crate::models::GetTransacEmailsList**](getTransacEmailsList.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_template

> crate::models::SendTemplateEmail send_template(template_id, send_email)
Send a template

This endpoint is deprecated. Prefer v3/smtp/email instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i64** | Id of the template | [required] |
**send_email** | [**SendEmail**](SendEmail.md) |  | [required] |

### Return type

[**crate::models::SendTemplateEmail**](sendTemplateEmail.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_test_template

> send_test_template(template_id, send_test_email)
Send a template to your test list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i64** | Id of the template | [required] |
**send_test_email** | [**SendTestEmail**](SendTestEmail.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_transac_email

> crate::models::CreateSmtpEmail send_transac_email(send_smtp_email)
Send a transactional email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_smtp_email** | [**SendSmtpEmail**](SendSmtpEmail.md) | Values to send a transactional email | [required] |

### Return type

[**crate::models::CreateSmtpEmail**](createSmtpEmail.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## smtp_blocked_contacts_email_delete

> smtp_blocked_contacts_email_delete(email)
Unblock or resubscribe a transactional contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | contact email (urlencoded) to unblock. | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## smtp_log_message_id_delete

> smtp_log_message_id_delete(message_id)
Delete an SMTP transactional log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | MessageId of the transactional log to delete | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_smtp_template

> update_smtp_template(template_id, smtp_template)
Update an email template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i64** | id of the template | [required] |
**smtp_template** | [**UpdateSmtpTemplate**](UpdateSmtpTemplate.md) | values to update in transactional email template | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

