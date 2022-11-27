# GetExtendedCampaignOverview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | ID of the campaign | 
**name** | **String** | Name of the campaign | 
**subject** | Option<**String**> | Subject of the campaign. Only available if `abTesting` flag of the campaign is `false` | [optional]
**r#type** | **String** | Type of campaign | 
**status** | **String** | Status of the campaign | 
**scheduled_at** | Option<**String**> | UTC date-time on which campaign is scheduled (YYYY-MM-DDTHH:mm:ss.SSSZ) | [optional]
**ab_testing** | Option<**bool**> | Status of A/B Test for the campaign. abTesting = false means it is disabled, & abTesting = true means it is enabled. | [optional]
**subject_a** | Option<**String**> | Subject A of the ab-test campaign. Only available if `abTesting` flag of the campaign is `true` | [optional]
**subject_b** | Option<**String**> | Subject B of the ab-test campaign. Only available if `abTesting` flag of the campaign is `true` | [optional]
**split_rule** | Option<**i32**> | The size of your ab-test groups. Only available if `abTesting` flag of the campaign is `true` | [optional]
**winner_criteria** | Option<**String**> | Criteria for the winning version. Only available if `abTesting` flag of the campaign is `true` | [optional]
**winner_delay** | Option<**i32**> | The duration of the test in hours at the end of which the winning version will be sent. Only available if `abTesting` flag of the campaign is `true` | [optional]
**send_at_best_time** | Option<**bool**> | It is true if you have chosen to send your campaign at best time, otherwise it is false | [optional]
**test_sent** | **bool** | Retrieved the status of test email sending. (true=Test email has been sent  false=Test email has not been sent) | 
**header** | **String** | Header of the campaign | 
**footer** | **String** | Footer of the campaign | 
**sender** | [**crate::models::GetExtendedCampaignOverviewAllOfSender**](getExtendedCampaignOverview_allOf_sender.md) |  | 
**reply_to** | **String** | Email defined as the \"Reply to\" of the campaign | 
**to_field** | **String** | Customisation of the \"to\" field of the campaign | 
**html_content** | **String** | HTML content of the campaign | 
**share_link** | Option<**String**> | Link to share the campaign on social medias | [optional]
**tag** | **String** | Tag of the campaign | 
**created_at** | **String** | Creation UTC date-time of the campaign (YYYY-MM-DDTHH:mm:ss.SSSZ) | 
**modified_at** | **String** | UTC date-time of last modification of the campaign (YYYY-MM-DDTHH:mm:ss.SSSZ) | 
**inline_image_activation** | Option<**bool**> | Status of inline image. inlineImageActivation = false means image can’t be embedded, & inlineImageActivation = true means image can be embedded, in the email. | [optional]
**mirror_active** | Option<**bool**> | Status of mirror links in campaign. mirrorActive = false means mirror links are deactivated, & mirrorActive = true means mirror links are activated, in the campaign | [optional]
**recurring** | Option<**bool**> | FOR TRIGGER ONLY ! Type of trigger campaign.recurring = false means contact can receive the same Trigger campaign only once, & recurring = true means contact can receive the same Trigger campaign several times | [optional]
**sent_date** | Option<**String**> | Sent UTC date-time of the campaign (YYYY-MM-DDTHH:mm:ss.SSSZ). Only available if 'status' of the campaign is 'sent' | [optional]
**return_bounce** | Option<**i64**> | Total number of non-delivered campaigns for a particular campaign id. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


