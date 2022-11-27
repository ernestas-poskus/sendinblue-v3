# SendSmtpEmail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sender** | Option<[**crate::models::SendSmtpEmailSender**](sendSmtpEmail_sender.md)> |  | [optional]
**to** | [**Vec<crate::models::SendSmtpEmailToInner>**](sendSmtpEmail_to_inner.md) | List of email addresses and names (optional) of the recipients. For example, [{\"name\":\"Jimmy\", \"email\":\"jimmy98@example.com\"}, {\"name\":\"Joe\", \"email\":\"joe@example.com\"}] | 
**bcc** | Option<[**Vec<crate::models::SendSmtpEmailBccInner>**](sendSmtpEmail_bcc_inner.md)> | List of email addresses and names (optional) of the recipients in bcc | [optional]
**cc** | Option<[**Vec<crate::models::SendSmtpEmailCcInner>**](sendSmtpEmail_cc_inner.md)> | List of email addresses and names (optional) of the recipients in cc | [optional]
**html_content** | Option<**String**> | HTML body of the message ( Mandatory if 'templateId' is not passed, ignored if 'templateId' is passed ) | [optional]
**text_content** | Option<**String**> | Plain Text body of the message ( Ignored if 'templateId' is passed ) | [optional]
**subject** | Option<**String**> | Subject of the message. Mandatory if 'templateId' is not passed | [optional]
**reply_to** | Option<[**crate::models::SendSmtpEmailReplyTo**](sendSmtpEmail_replyTo.md)> |  | [optional]
**attachment** | Option<[**Vec<crate::models::SendSmtpEmailAttachmentInner>**](sendSmtpEmail_attachment_inner.md)> | Pass the absolute URL (no local file) or the base64 content of the attachment along with the attachment name (Mandatory if attachment content is passed). For example, `[{\"url\":\"https://attachment.domain.com/myAttachmentFromUrl.jpg\", \"name\":\"myAttachmentFromUrl.jpg\"}, {\"content\":\"base64 example content\", \"name\":\"myAttachmentFromBase64.jpg\"}]`. Allowed extensions for attachment file: xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub, eps, odt, mp3, m4a, m4v, wma, ogg, flac, wav, aif, aifc, aiff, mp4, mov, avi, mkv, mpeg, mpg and wmv ( If 'templateId' is passed and is in New Template Language format then both attachment url and content are accepted. If template is in Old template Language format, then 'attachment' is ignored ) | [optional]
**headers** | Option<[**serde_json::Value**](.md)> | Pass the set of custom headers (not the standard headers) that shall be sent along the mail headers in the original email. 'sender.ip' header can be set (only for dedicated ip users) to mention the IP to be used for sending transactional emails. Headers are allowed in `This-Case-Only` (i.e. words separated by hyphen with first letter of each word in capital letter), they will be converted to such case styling if not in this format in the request payload. For example, `{\"sender.ip\":\"1.2.3.4\", \"X-Mailin-custom\":\"some_custom_header\", \"idempotencyKey\":\"abc-123\"}`. | [optional]
**template_id** | Option<**i64**> | Id of the template | [optional]
**params** | Option<[**serde_json::Value**](.md)> | Pass the set of attributes to customize the template. For example, {\"FNAME\":\"Joe\", \"LNAME\":\"Doe\"}. It's considered only if template is in New Template Language format. | [optional]
**tags** | Option<**Vec<String>**> | Tag your emails to find them more easily | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


