# GetCampaignStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**list_id** | Option<**i64**> | List Id of email campaign (only in case of get email campaign(s)(not for global stats)) | [optional]
**unique_clicks** | **i64** | Number of unique clicks for the campaign | 
**clickers** | **i64** | Number of total clicks for the campaign | 
**complaints** | **i64** | Number of complaints (Spam reports) for the campaign | 
**delivered** | **i64** | Number of delivered emails for the campaign | 
**sent** | **i64** | Number of sent emails for the campaign | 
**soft_bounces** | **i64** | Number of softbounce for the campaign | 
**hard_bounces** | **i64** | Number of harbounce for the campaign | 
**unique_views** | **i64** | Number of unique openings for the campaign | 
**trackable_views** | **i64** | Recipients without any privacy protection option enabled in their email client | 
**unsubscriptions** | **i64** | Number of unsubscription for the campaign | 
**viewed** | **i64** | Number of openings for the campaign | 
**deferred** | Option<**i64**> | Number of deferred emails for the campaign | [optional]
**return_bounce** | Option<**i64**> | Total number of non-delivered campaigns for a particular campaign id. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


