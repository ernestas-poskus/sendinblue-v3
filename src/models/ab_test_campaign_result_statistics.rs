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
pub struct AbTestCampaignResultStatistics {
    #[serde(rename = "openers")]
    pub openers: Box<crate::models::AbTestVersionStats>,
    #[serde(rename = "clicks")]
    pub clicks: Box<crate::models::AbTestVersionStats>,
    #[serde(rename = "unsubscribed")]
    pub unsubscribed: Box<crate::models::AbTestVersionStats>,
    #[serde(rename = "hardBounces")]
    pub hard_bounces: Box<crate::models::AbTestVersionStats>,
    #[serde(rename = "softBounces")]
    pub soft_bounces: Box<crate::models::AbTestVersionStats>,
    #[serde(rename = "complaints")]
    pub complaints: Box<crate::models::AbTestVersionStats>,
}

impl AbTestCampaignResultStatistics {
    pub fn new(openers: crate::models::AbTestVersionStats, clicks: crate::models::AbTestVersionStats, unsubscribed: crate::models::AbTestVersionStats, hard_bounces: crate::models::AbTestVersionStats, soft_bounces: crate::models::AbTestVersionStats, complaints: crate::models::AbTestVersionStats) -> AbTestCampaignResultStatistics {
        AbTestCampaignResultStatistics {
            openers: Box::new(openers),
            clicks: Box::new(clicks),
            unsubscribed: Box::new(unsubscribed),
            hard_bounces: Box::new(hard_bounces),
            soft_bounces: Box::new(soft_bounces),
            complaints: Box::new(complaints),
        }
    }
}


