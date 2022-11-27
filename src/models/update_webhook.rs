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
pub struct UpdateWebhook {
    /// URL of the webhook
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Description of the webhook
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Events triggering the webhook. Possible values for Transactional type webhook – `sent` OR `request`, `delivered`, `hardBounce`, `softBounce`, `blocked`, `spam`, `invalid`, `deferred`, `click`, `opened`, `uniqueOpened` and `unsubscribed` and possible values for Marketing type webhook – `spam`, `opened`, `click`, `hardBounce`, `softBounce`, `unsubscribed`, `listAddition` and `delivered`
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Events>>,
}

impl UpdateWebhook {
    pub fn new() -> UpdateWebhook {
        UpdateWebhook {
            url: None,
            description: None,
            events: None,
        }
    }
}

/// Events triggering the webhook. Possible values for Transactional type webhook – `sent` OR `request`, `delivered`, `hardBounce`, `softBounce`, `blocked`, `spam`, `invalid`, `deferred`, `click`, `opened`, `uniqueOpened` and `unsubscribed` and possible values for Marketing type webhook – `spam`, `opened`, `click`, `hardBounce`, `softBounce`, `unsubscribed`, `listAddition` and `delivered`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Events {
    #[serde(rename = "hardBounce")]
    HardBounce,
    #[serde(rename = "softBounce")]
    SoftBounce,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "deferred")]
    Deferred,
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "uniqueOpened")]
    UniqueOpened,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "listAddition")]
    ListAddition,
    #[serde(rename = "contactUpdated")]
    ContactUpdated,
    #[serde(rename = "contactDeleted")]
    ContactDeleted,
}

impl Default for Events {
    fn default() -> Events {
        Self::HardBounce
    }
}
