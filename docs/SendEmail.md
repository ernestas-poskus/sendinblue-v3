# SendEmail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email_to** | **Vec<String>** | List of the email addresses of the recipients. For example, ['abc@example.com', 'asd@example.com']. | 
**email_bcc** | Option<**Vec<String>**> | List of the email addresses of the recipients in bcc | [optional]
**email_cc** | Option<**Vec<String>**> | List of the email addresses of the recipients in cc | [optional]
**reply_to** | Option<**String**> | Email address which shall be used by campaign recipients to reply back | [optional]
**attachment_url** | Option<**String**> | Absolute url of the attachment (no local file). Extension allowed: xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps | [optional]
**attachment** | Option<[**Vec<crate::models::SendEmailAttachmentInner>**](sendEmail_attachment_inner.md)> | Pass the list of content (base64 encoded) and name of the attachment. For example, [{\"content\":\"base64 encoded content 1\", \"name\":\"attcahment1\"}, {\"content\":\"base64 encoded content 2\", \"name\":\"attcahment2\"}]. | [optional]
**headers** | Option<[**serde_json::Value**](.md)> | Pass the set of headers that shall be sent along the mail headers in the original email. 'sender.ip' header can be set (only for dedicated ip users) to mention the IP to be used for sending transactional emails. Headers are allowed in `This-Case-Only` (i.e. words separated by hyphen with first letter of each word in capital letter), they will be converted to such case styling if not in this format in the request payload. For example, {\"Content-Type\":\"text/html\", \"charset\":\"iso-8859-1\", \"sender.ip\":\"1.2.3.4\"} | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> | Pass the set of attributes to customize the template. For example, {\"FNAME\":\"Joe\", \"LNAME\":\"Doe\"} | [optional]
**tags** | Option<**Vec<String>**> | Tag your emails to find them more easily | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


