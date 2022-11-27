/*
 * SendinBlue API
 *
 * SendinBlue provide a RESTFul API that can be used with any languages. With this API, you will be able to :   - Manage your campaigns and get the statistics   - Manage your contacts   - Send transactional Emails and SMS   - and much more...  You can download our wrappers at https://github.com/orgs/sendinblue  **Possible responses**   | Code | Message |   | :-------------: | ------------- |   | 200  | OK. Successful Request  |   | 201  | OK. Successful Creation |   | 202  | OK. Request accepted |   | 204  | OK. Successful Update/Deletion  |   | 400  | Error. Bad Request  |   | 401  | Error. Authentication Needed  |   | 402  | Error. Not enough credit, plan upgrade needed  |   | 403  | Error. Permission denied  |   | 404  | Error. Object does not exist |   | 405  | Error. Method not allowed  |   | 406  | Error. Not Acceptable  | 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: contact@sendinblue.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SendEmail {
    /// List of the email addresses of the recipients. For example, ['abc@example.com', 'asd@example.com'].
    #[serde(rename = "emailTo")]
    pub email_to: Vec<String>,
    /// List of the email addresses of the recipients in bcc
    #[serde(rename = "emailBcc", skip_serializing_if = "Option::is_none")]
    pub email_bcc: Option<Vec<String>>,
    /// List of the email addresses of the recipients in cc
    #[serde(rename = "emailCc", skip_serializing_if = "Option::is_none")]
    pub email_cc: Option<Vec<String>>,
    /// Email address which shall be used by campaign recipients to reply back
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Absolute url of the attachment (no local file). Extension allowed: xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps
    #[serde(rename = "attachmentUrl", skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,
    /// Pass the list of content (base64 encoded) and name of the attachment. For example, [{\"content\":\"base64 encoded content 1\", \"name\":\"attcahment1\"}, {\"content\":\"base64 encoded content 2\", \"name\":\"attcahment2\"}].
    #[serde(rename = "attachment", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<crate::models::SendEmailAttachmentInner>>,
    /// Pass the set of headers that shall be sent along the mail headers in the original email. 'sender.ip' header can be set (only for dedicated ip users) to mention the IP to be used for sending transactional emails. Headers are allowed in `This-Case-Only` (i.e. words separated by hyphen with first letter of each word in capital letter), they will be converted to such case styling if not in this format in the request payload. For example, {\"Content-Type\":\"text/html\", \"charset\":\"iso-8859-1\", \"sender.ip\":\"1.2.3.4\"}
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// Pass the set of attributes to customize the template. For example, {\"FNAME\":\"Joe\", \"LNAME\":\"Doe\"}
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    /// Tag your emails to find them more easily
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl SendEmail {
    pub fn new(email_to: Vec<String>) -> SendEmail {
        SendEmail {
            email_to,
            email_bcc: None,
            email_cc: None,
            reply_to: None,
            attachment_url: None,
            attachment: None,
            headers: None,
            attributes: None,
            tags: None,
        }
    }
}


