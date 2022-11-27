# GetSmsEventReportEventsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | **String** | Phone number which has generated the event | 
**date** | **String** | UTC date-time on which the event has been generated | 
**message_id** | **String** | Message ID which generated the event | 
**event** | **String** | Event which occurred | 
**reason** | Option<**String**> | Reason of bounce (only available if the event is hardbounce or softbounce) | [optional]
**reply** | Option<**String**> |  | [optional]
**tag** | Option<**String**> | Tag of the SMS which generated the event | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


