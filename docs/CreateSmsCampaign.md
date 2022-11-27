# CreateSmsCampaign

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the campaign | 
**sender** | **String** | Name of the sender. **The number of characters is limited to 11 for alphanumeric characters and 15 for numeric characters** | 
**content** | **String** | Content of the message. The maximum characters used per SMS is 160, if used more than that, it will be counted as more than one SMS | 
**recipients** | Option<[**crate::models::CreateSmsCampaignRecipients**](createSmsCampaign_recipients.md)> |  | [optional]
**scheduled_at** | Option<**String**> | UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


