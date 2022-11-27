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
pub struct GetTransacSmsReportReportsInner {
    /// Date for which statistics are retrieved
    #[serde(rename = "date")]
    pub date: String,
    /// Number of requests for the date
    #[serde(rename = "requests")]
    pub requests: i64,
    /// Number of delivered SMS for the date
    #[serde(rename = "delivered")]
    pub delivered: i64,
    /// Number of hardbounces for the date
    #[serde(rename = "hardBounces")]
    pub hard_bounces: i64,
    /// Number of softbounces for the date
    #[serde(rename = "softBounces")]
    pub soft_bounces: i64,
    /// Number of blocked contact for the date
    #[serde(rename = "blocked")]
    pub blocked: i64,
    /// Number of unsubscription for the date
    #[serde(rename = "unsubscribed")]
    pub unsubscribed: i64,
    /// Number of answered SMS for the date
    #[serde(rename = "replied")]
    pub replied: i64,
    /// Number of accepted for the date
    #[serde(rename = "accepted")]
    pub accepted: i64,
    /// Number of rejected for the date
    #[serde(rename = "rejected")]
    pub rejected: i64,
}

impl GetTransacSmsReportReportsInner {
    pub fn new(date: String, requests: i64, delivered: i64, hard_bounces: i64, soft_bounces: i64, blocked: i64, unsubscribed: i64, replied: i64, accepted: i64, rejected: i64) -> GetTransacSmsReportReportsInner {
        GetTransacSmsReportReportsInner {
            date,
            requests,
            delivered,
            hard_bounces,
            soft_bounces,
            blocked,
            unsubscribed,
            replied,
            accepted,
            rejected,
        }
    }
}


