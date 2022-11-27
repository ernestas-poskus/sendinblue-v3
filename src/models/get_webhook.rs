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
pub struct GetWebhook {
    /// URL of the webhook
    #[serde(rename = "url")]
    pub url: String,
    /// ID of the webhook
    #[serde(rename = "id")]
    pub id: i64,
    /// Description of the webhook
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "events")]
    pub events: Vec<String>,
    /// Type of webhook (marketing or transac)
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// Creation UTC date-time of the webhook (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Last modification UTC date-time of the webhook (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
}

impl GetWebhook {
    pub fn new(url: String, id: i64, description: String, events: Vec<String>, r#type: RHashType, created_at: String, modified_at: String) -> GetWebhook {
        GetWebhook {
            url,
            id,
            description,
            events,
            r#type,
            created_at,
            modified_at,
        }
    }
}

/// Type of webhook (marketing or transac)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "marketing")]
    Marketing,
    #[serde(rename = "transac")]
    Transac,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Marketing
    }
}

