# GetEmailEventReportEventsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Email address which generates the event | 
**date** | **String** | UTC date-time on which the event has been generated | 
**subject** | Option<**String**> | Subject of the event | [optional]
**message_id** | **String** | Message ID which generated the event | 
**event** | **String** | Event which occurred | 
**reason** | Option<**String**> | Reason of bounce (only available if the event is hardbounce or softbounce) | [optional]
**tag** | Option<**String**> | Tag of the email which generated the event | [optional]
**ip** | Option<**String**> | IP from which the user has opened the email or clicked on the link (only available if the event is opened or clicks) | [optional]
**link** | Option<**String**> | The link which is sent to the user (only available if the event is requests or opened or clicks) | [optional]
**from** | Option<**String**> | Sender email from which the emails are sent | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


