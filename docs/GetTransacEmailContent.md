# GetTransacEmailContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Email address to which transactional email has been sent | 
**subject** | **String** | Subject of the sent email | 
**template_id** | Option<**i64**> | Id of the template | [optional]
**date** | **String** | Date on which transactional email was sent | 
**events** | [**Vec<crate::models::GetTransacEmailContentEventsInner>**](getTransacEmailContent_events_inner.md) | Series of events which occurred on the transactional email | 
**body** | **String** | Actual content of the transactional email that has been sent | 
**attachment_count** | **i64** | Count of the attachments that were sent in the email | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


