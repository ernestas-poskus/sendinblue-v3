# UpdateSmtpTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tag** | Option<**String**> | Tag of the template | [optional]
**sender** | Option<[**crate::models::UpdateSmtpTemplateSender**](updateSmtpTemplate_sender.md)> |  | [optional]
**template_name** | Option<**String**> | Name of the template | [optional]
**html_content** | Option<**String**> | Required if htmlUrl is empty. Body of the message (HTML must have more than 10 characters) | [optional]
**html_url** | Option<**String**> | Required if htmlContent is empty. URL to the body of the email (HTML) | [optional]
**subject** | Option<**String**> | Subject of the email | [optional]
**reply_to** | Option<**String**> | Email on which campaign recipients will be able to reply to | [optional]
**to_field** | Option<**String**> | To personalize the «To» Field. If you want to include the first name and last name of your recipient, add {FNAME} {LNAME}. These contact attributes must already exist in your SendinBlue account. If input parameter 'params' used please use {{contact.FNAME}} {{contact.LNAME}} for personalization | [optional]
**attachment_url** | Option<**String**> | Absolute url of the attachment (no local file). Extension allowed: xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps | [optional]
**is_active** | Option<**bool**> | Status of the template. isActive = false means template is inactive, isActive = true means template is active | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


