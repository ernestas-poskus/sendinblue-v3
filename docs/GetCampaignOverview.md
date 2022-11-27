# GetCampaignOverview

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

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


