# AbTestCampaignResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**winning_version** | Option<**String**> | Winning Campaign Info. pending = Campaign has been picked for sending and winning version is yet to be decided, tie = A tie happened between both the versions, notAvailable = Campaign has not yet been picked for sending. | [optional]
**winning_criteria** | Option<**String**> | Criteria choosen for winning version (Open/Click) | [optional]
**winning_subject_line** | Option<**String**> | Subject Line of current winning version | [optional]
**open_rate** | Option<**String**> | Open rate for current winning version | [optional]
**click_rate** | Option<**String**> | Click rate for current winning version | [optional]
**winning_version_rate** | Option<**String**> | Open/Click rate for the winner version | [optional]
**statistics** | Option<[**crate::models::AbTestCampaignResultStatistics**](abTestCampaignResult_statistics.md)> |  | [optional]
**clicked_links** | Option<[**crate::models::AbTestCampaignResultClickedLinks**](abTestCampaignResult_clickedLinks.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


