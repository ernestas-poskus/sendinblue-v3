# GetSmsCampaign

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | ID of the SMS Campaign | 
**name** | **String** | Name of the SMS Campaign | 
**status** | **String** | Status of the SMS Campaign | 
**content** | **String** | Content of the SMS Campaign | 
**scheduled_at** | **String** | UTC date-time on which SMS campaign is scheduled. Should be in YYYY-MM-DDTHH:mm:ss.SSSZ format | 
**sender** | **String** | Sender of the SMS Campaign | 
**created_at** | **String** | Creation UTC date-time of the SMS campaign (YYYY-MM-DDTHH:mm:ss.SSSZ) | 
**modified_at** | **String** | UTC date-time of last modification of the SMS campaign (YYYY-MM-DDTHH:mm:ss.SSSZ) | 
**recipients** | [**crate::models::GetCampaignRecipients**](getCampaignRecipients.md) |  | 
**statistics** | [**crate::models::GetSmsCampaignStats**](getSmsCampaignStats.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


