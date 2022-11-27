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
pub struct GetExtendedCampaignStatsCampaignStatsInner {
    /// List Id of email campaign (only in case of get email campaign(s)(not for global stats))
    #[serde(rename = "listId", skip_serializing_if = "Option::is_none")]
    pub list_id: Option<i64>,
    /// Number of unique clicks for the campaign
    #[serde(rename = "uniqueClicks")]
    pub unique_clicks: i64,
    /// Number of total clicks for the campaign
    #[serde(rename = "clickers")]
    pub clickers: i64,
    /// Number of complaints (Spam reports) for the campaign
    #[serde(rename = "complaints")]
    pub complaints: i64,
    /// Number of delivered emails for the campaign
    #[serde(rename = "delivered")]
    pub delivered: i64,
    /// Number of sent emails for the campaign
    #[serde(rename = "sent")]
    pub sent: i64,
    /// Number of softbounce for the campaign
    #[serde(rename = "softBounces")]
    pub soft_bounces: i64,
    /// Number of harbounce for the campaign
    #[serde(rename = "hardBounces")]
    pub hard_bounces: i64,
    /// Number of unique openings for the campaign
    #[serde(rename = "uniqueViews")]
    pub unique_views: i64,
    /// Recipients without any privacy protection option enabled in their email client
    #[serde(rename = "trackableViews")]
    pub trackable_views: i64,
    /// Number of unsubscription for the campaign
    #[serde(rename = "unsubscriptions")]
    pub unsubscriptions: i64,
    /// Number of openings for the campaign
    #[serde(rename = "viewed")]
    pub viewed: i64,
    /// Number of deferred emails for the campaign
    #[serde(rename = "deferred", skip_serializing_if = "Option::is_none")]
    pub deferred: Option<i64>,
    /// Total number of non-delivered campaigns for a particular campaign id.
    #[serde(rename = "returnBounce", skip_serializing_if = "Option::is_none")]
    pub return_bounce: Option<i64>,
}

impl GetExtendedCampaignStatsCampaignStatsInner {
    pub fn new(unique_clicks: i64, clickers: i64, complaints: i64, delivered: i64, sent: i64, soft_bounces: i64, hard_bounces: i64, unique_views: i64, trackable_views: i64, unsubscriptions: i64, viewed: i64) -> GetExtendedCampaignStatsCampaignStatsInner {
        GetExtendedCampaignStatsCampaignStatsInner {
            list_id: None,
            unique_clicks,
            clickers,
            complaints,
            delivered,
            sent,
            soft_bounces,
            hard_bounces,
            unique_views,
            trackable_views,
            unsubscriptions,
            viewed,
            deferred: None,
            return_bounce: None,
        }
    }
}


