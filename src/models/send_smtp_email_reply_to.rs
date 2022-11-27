/*
 * SendinBlue API
 *
 * SendinBlue provide a RESTFul API that can be used with any languages. With this API, you will be able to :   - Manage your campaigns and get the statistics   - Manage your contacts   - Send transactional Emails and SMS   - and much more...  You can download our wrappers at https://github.com/orgs/sendinblue  **Possible responses**   | Code | Message |   | :-------------: | ------------- |   | 200  | OK. Successful Request  |   | 201  | OK. Successful Creation |   | 202  | OK. Request accepted |   | 204  | OK. Successful Update/Deletion  |   | 400  | Error. Bad Request  |   | 401  | Error. Authentication Needed  |   | 402  | Error. Not enough credit, plan upgrade needed  |   | 403  | Error. Permission denied  |   | 404  | Error. Object does not exist |   | 405  | Error. Method not allowed  |   | 406  | Error. Not Acceptable  | 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: contact@sendinblue.com
 * Generated by: https://openapi-generator.tech
 */

/// SendSmtpEmailReplyTo : Email (required), along with name (optional), on which transactional mail recipients will be able to reply back. For example, {\"email\":\"ann6533@example.com\", \"name\":\"Ann\"}.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SendSmtpEmailReplyTo {
    /// Email address in reply to
    #[serde(rename = "email")]
    pub email: String,
    /// Name in reply to. Maximum allowed characters are 70.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SendSmtpEmailReplyTo {
    /// Email (required), along with name (optional), on which transactional mail recipients will be able to reply back. For example, {\"email\":\"ann6533@example.com\", \"name\":\"Ann\"}.
    pub fn new(email: String) -> SendSmtpEmailReplyTo {
        SendSmtpEmailReplyTo {
            email,
            name: None,
        }
    }
}


