# CreateEmailCampaign

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tag** | Option<**String**> | Tag of the campaign | [optional]
**sender** | [**crate::models::CreateEmailCampaignSender**](createEmailCampaign_sender.md) |  | 
**name** | **String** | Name of the campaign | 
**html_content** | Option<**String**> | Mandatory if htmlUrl and templateId are empty. Body of the message (HTML) | [optional]
**html_url** | Option<**String**> | Mandatory if htmlContent and templateId are empty. Url to the message (HTML) | [optional]
**template_id** | Option<**i64**> | Mandatory if htmlContent and htmlUrl are empty. Id of the transactional email template with status 'active'. Used to copy only its content fetched from htmlContent/htmlUrl to an email campaign for RSS feature. | [optional]
**scheduled_at** | Option<**String**> | Sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result. If sendAtBestTime is set to true, your campaign will be sent according to the date passed (ignoring the time part). | [optional]
**subject** | Option<**String**> | Subject of the campaign. Mandatory if abTesting is false. Ignored if abTesting is true. | [optional]
**reply_to** | Option<**String**> | Email on which the campaign recipients will be able to reply to | [optional]
**to_field** | Option<**String**> | To personalize the «To» Field. If you want to include the first name and last name of your recipient, add {FNAME} {LNAME}. These contact attributes must already exist in your SendinBlue account. If input parameter 'params' used please use {{contact.FNAME}} {{contact.LNAME}} for personalization | [optional]
**recipients** | Option<[**crate::models::CreateEmailCampaignRecipients**](createEmailCampaign_recipients.md)> |  | [optional]
**attachment_url** | Option<**String**> | Absolute url of the attachment (no local file). Extension allowed: xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps | [optional]
**inline_image_activation** | Option<**bool**> | Use true to embedded the images in your email. Final size of the email should be less than 4MB. Campaigns with embedded images can not be sent to more than 5000 contacts | [optional][default to false]
**mirror_active** | Option<**bool**> | Use true to enable the mirror link | [optional]
**footer** | Option<**String**> | Footer of the email campaign | [optional]
**header** | Option<**String**> | Header of the email campaign | [optional]
**utm_campaign** | Option<**String**> | Customize the utm_campaign value. If this field is empty, the campaign name will be used. Only alphanumeric characters and spaces are allowed | [optional]
**params** | Option<[**serde_json::Value**](.md)> | Pass the set of attributes to customize the type classic campaign. For example, {\"FNAME\":\"Joe\", \"LNAME\":\"Doe\"}. Only available if 'type' is 'classic'. It's considered only if campaign is in New Template Language format. The New Template Language is dependent on the values of 'subject', 'htmlContent/htmlUrl', 'sender.name' & 'toField' | [optional]
**send_at_best_time** | Option<**bool**> | Set this to true if you want to send your campaign at best time. | [optional][default to false]
**ab_testing** | Option<**bool**> | Status of A/B Test. abTesting = false means it is disabled, & abTesting = true means it is enabled. 'subjectA', 'subjectB', 'splitRule', 'winnerCriteria' & 'winnerDelay' will be considered when abTesting is set to true. 'subjectA' & 'subjectB' are mandatory together & 'subject' if passed is ignored. Can be set to true only if 'sendAtBestTime' is 'false'. You will be able to set up two subject lines for your campaign and send them to a random sample of your total recipients. Half of the test group will receive version A, and the other half will receive version B | [optional][default to false]
**subject_a** | Option<**String**> | Subject A of the campaign. Mandatory if abTesting = true. subjectA & subjectB should have unique value | [optional]
**subject_b** | Option<**String**> | Subject B of the campaign. Mandatory if abTesting = true. subjectA & subjectB should have unique value | [optional]
**split_rule** | Option<**i64**> | Add the size of your test groups. Mandatory if abTesting = true & 'recipients' is passed. We'll send version A and B to a random sample of recipients, and then the winning version to everyone else | [optional]
**winner_criteria** | Option<**String**> | Choose the metrics that will determinate the winning version. Mandatory if 'splitRule' >= 1 and < 50. If splitRule = 50, 'winnerCriteria' is ignored if passed | [optional]
**winner_delay** | Option<**i64**> | Choose the duration of the test in hours. Maximum is 7 days, pass 24*7 = 168 hours. The winning version will be sent at the end of the test. Mandatory if 'splitRule' >= 1 and < 50. If splitRule = 50, 'winnerDelay' is ignored if passed | [optional]
**ip_warmup_enable** | Option<**bool**> | Available for dedicated ip clients. Set this to true if you wish to warm up your ip. | [optional][default to false]
**initial_quota** | Option<**i64**> | Mandatory if ipWarmupEnable is set to true. Set an initial quota greater than 1 for warming up your ip. We recommend you set a value of 3000. | [optional]
**increase_rate** | Option<**i64**> | Mandatory if ipWarmupEnable is set to true. Set a percentage increase rate for warming up your ip. We recommend you set the increase rate to 30% per day. If you want to send the same number of emails every day, set the daily increase value to 0%. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


