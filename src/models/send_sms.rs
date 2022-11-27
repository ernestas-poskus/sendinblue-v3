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
pub struct SendSms {
    #[serde(rename = "reference")]
    pub reference: String,
    #[serde(rename = "messageId")]
    pub message_id: i64,
    /// Count of SMS's to send multiple text messages
    #[serde(rename = "smsCount", skip_serializing_if = "Option::is_none")]
    pub sms_count: Option<i64>,
    /// SMS credits used per text message
    #[serde(rename = "usedCredits", skip_serializing_if = "Option::is_none")]
    pub used_credits: Option<f32>,
    /// Remaining SMS credits of the user
    #[serde(rename = "remainingCredits", skip_serializing_if = "Option::is_none")]
    pub remaining_credits: Option<f32>,
}

impl SendSms {
    pub fn new(reference: String, message_id: i64) -> SendSms {
        SendSms {
            reference,
            message_id,
            sms_count: None,
            used_credits: None,
            remaining_credits: None,
        }
    }
}

