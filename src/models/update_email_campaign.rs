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
pub struct UpdateEmailCampaign {
    /// Tag of the campaign
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<crate::models::UpdateEmailCampaignSender>>,
    /// Name of the campaign
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Body of the message (HTML version). REQUIRED if htmlUrl is empty
    #[serde(rename = "htmlContent", skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// Url which contents the body of the email message. REQUIRED if htmlContent is empty
    #[serde(rename = "htmlUrl", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    /// UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result. If sendAtBestTime is set to true, your campaign will be sent according to the date passed (ignoring the time part).
    #[serde(rename = "scheduledAt", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    /// Subject of the campaign
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Email on which campaign recipients will be able to reply to
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// To personalize the «To» Field. If you want to include the first name and last name of your recipient, add {FNAME} {LNAME}. These contact attributes must already exist in your SendinBlue account. If input parameter 'params' used please use {{contact.FNAME}} {{contact.LNAME}} for personalization
    #[serde(rename = "toField", skip_serializing_if = "Option::is_none")]
    pub to_field: Option<String>,
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Box<crate::models::UpdateEmailCampaignRecipients>>,
    /// Absolute url of the attachment (no local file). Extension allowed: xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps
    #[serde(rename = "attachmentUrl", skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,
    /// Status of inline image. inlineImageActivation = false means image can’t be embedded, & inlineImageActivation = true means image can be embedded, in the email. You cannot send a campaign of more than 4MB with images embedded in the email. Campaigns with the images embedded in the email must be sent to less than 5000 contacts.
    #[serde(rename = "inlineImageActivation", skip_serializing_if = "Option::is_none")]
    pub inline_image_activation: Option<bool>,
    /// Status of mirror links in campaign. mirrorActive = false means mirror links are deactivated, & mirrorActive = true means mirror links are activated, in the campaign
    #[serde(rename = "mirrorActive", skip_serializing_if = "Option::is_none")]
    pub mirror_active: Option<bool>,
    /// FOR TRIGGER ONLY ! Type of trigger campaign.recurring = false means contact can receive the same Trigger campaign only once, & recurring = true means contact can receive the same Trigger campaign several times
    #[serde(rename = "recurring", skip_serializing_if = "Option::is_none")]
    pub recurring: Option<bool>,
    /// Footer of the email campaign
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// Header of the email campaign
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    /// Customize the utm_campaign value. If this field is empty, the campaign name will be used. Only alphanumeric characters and spaces are allowed
    #[serde(rename = "utmCampaign", skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Option<String>,
    /// Pass the set of attributes to customize the type 'classic' campaign. For example, {\"FNAME\":\"Joe\", \"LNAME\":\"Doe\"}. The 'params' field will get updated, only if the campaign is in New Template Language, else ignored. The New Template Language is dependent on the values of 'subject', 'htmlContent/htmlUrl', 'sender.name' & 'toField'
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    /// Set this to true if you want to send your campaign at best time. Note:- if true, warmup ip will be disabled.
    #[serde(rename = "sendAtBestTime", skip_serializing_if = "Option::is_none")]
    pub send_at_best_time: Option<bool>,
    /// Status of A/B Test. abTesting = false means it is disabled, & abTesting = true means it is enabled. 'subjectA', 'subjectB', 'splitRule', 'winnerCriteria' & 'winnerDelay' will be considered if abTesting is set to true. 'subject' if passed is ignored.  Can be set to true only if 'sendAtBestTime' is 'false'. You will be able to set up two subject lines for your campaign and send them to a random sample of your total recipients. Half of the test group will receive version A, and the other half will receive version B
    #[serde(rename = "abTesting", skip_serializing_if = "Option::is_none")]
    pub ab_testing: Option<bool>,
    /// Subject A of the campaign. Considered if abTesting = true. subjectA & subjectB should have unique value
    #[serde(rename = "subjectA", skip_serializing_if = "Option::is_none")]
    pub subject_a: Option<String>,
    /// Subject B of the campaign. Considered if abTesting = true. subjectA & subjectB should have unique value
    #[serde(rename = "subjectB", skip_serializing_if = "Option::is_none")]
    pub subject_b: Option<String>,
    /// Add the size of your test groups. Considered if abTesting = true. We'll send version A and B to a random sample of recipients, and then the winning version to everyone else
    #[serde(rename = "splitRule", skip_serializing_if = "Option::is_none")]
    pub split_rule: Option<i64>,
    /// Choose the metrics that will determinate the winning version. Considered if 'splitRule' >= 1 and < 50. If splitRule = 50, 'winnerCriteria' is ignored if passed or alreday exist in record
    #[serde(rename = "winnerCriteria", skip_serializing_if = "Option::is_none")]
    pub winner_criteria: Option<WinnerCriteria>,
    /// Choose the duration of the test in hours. Maximum is 7 days, pass 24*7 = 168 hours. The winning version will be sent at the end of the test. Considered if 'splitRule' >= 1 and < 50. If splitRule = 50, 'winnerDelay' is ignored if passed or alreday exist in record
    #[serde(rename = "winnerDelay", skip_serializing_if = "Option::is_none")]
    pub winner_delay: Option<i64>,
    /// Available for dedicated ip clients. Set this to true if you wish to warm up your ip.
    #[serde(rename = "ipWarmupEnable", skip_serializing_if = "Option::is_none")]
    pub ip_warmup_enable: Option<bool>,
    /// Set an initial quota greater than 1 for warming up your ip. We recommend you set a value of 3000.
    #[serde(rename = "initialQuota", skip_serializing_if = "Option::is_none")]
    pub initial_quota: Option<i64>,
    /// Set a percentage increase rate for warming up your ip. We recommend you set the increase rate to 30% per day. If you want to send the same number of emails every day, set the daily increase value to 0%.
    #[serde(rename = "increaseRate", skip_serializing_if = "Option::is_none")]
    pub increase_rate: Option<i64>,
}

impl UpdateEmailCampaign {
    pub fn new() -> UpdateEmailCampaign {
        UpdateEmailCampaign {
            tag: None,
            sender: None,
            name: None,
            html_content: None,
            html_url: None,
            scheduled_at: None,
            subject: None,
            reply_to: None,
            to_field: None,
            recipients: None,
            attachment_url: None,
            inline_image_activation: None,
            mirror_active: None,
            recurring: None,
            footer: None,
            header: None,
            utm_campaign: None,
            params: None,
            send_at_best_time: None,
            ab_testing: None,
            subject_a: None,
            subject_b: None,
            split_rule: None,
            winner_criteria: None,
            winner_delay: None,
            ip_warmup_enable: None,
            initial_quota: None,
            increase_rate: None,
        }
    }
}

/// Choose the metrics that will determinate the winning version. Considered if 'splitRule' >= 1 and < 50. If splitRule = 50, 'winnerCriteria' is ignored if passed or alreday exist in record
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WinnerCriteria {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "click")]
    Click,
}

impl Default for WinnerCriteria {
    fn default() -> WinnerCriteria {
        Self::Open
    }
}
