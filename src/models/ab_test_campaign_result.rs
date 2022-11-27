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
pub struct AbTestCampaignResult {
    /// Winning Campaign Info. pending = Campaign has been picked for sending and winning version is yet to be decided, tie = A tie happened between both the versions, notAvailable = Campaign has not yet been picked for sending.
    #[serde(rename = "winningVersion", skip_serializing_if = "Option::is_none")]
    pub winning_version: Option<WinningVersion>,
    /// Criteria choosen for winning version (Open/Click)
    #[serde(rename = "winningCriteria", skip_serializing_if = "Option::is_none")]
    pub winning_criteria: Option<WinningCriteria>,
    /// Subject Line of current winning version
    #[serde(rename = "winningSubjectLine", skip_serializing_if = "Option::is_none")]
    pub winning_subject_line: Option<String>,
    /// Open rate for current winning version
    #[serde(rename = "openRate", skip_serializing_if = "Option::is_none")]
    pub open_rate: Option<String>,
    /// Click rate for current winning version
    #[serde(rename = "clickRate", skip_serializing_if = "Option::is_none")]
    pub click_rate: Option<String>,
    /// Open/Click rate for the winner version
    #[serde(rename = "winningVersionRate", skip_serializing_if = "Option::is_none")]
    pub winning_version_rate: Option<String>,
    #[serde(rename = "statistics", skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Box<crate::models::AbTestCampaignResultStatistics>>,
    #[serde(rename = "clickedLinks", skip_serializing_if = "Option::is_none")]
    pub clicked_links: Option<Box<crate::models::AbTestCampaignResultClickedLinks>>,
}

impl AbTestCampaignResult {
    pub fn new() -> AbTestCampaignResult {
        AbTestCampaignResult {
            winning_version: None,
            winning_criteria: None,
            winning_subject_line: None,
            open_rate: None,
            click_rate: None,
            winning_version_rate: None,
            statistics: None,
            clicked_links: None,
        }
    }
}

/// Winning Campaign Info. pending = Campaign has been picked for sending and winning version is yet to be decided, tie = A tie happened between both the versions, notAvailable = Campaign has not yet been picked for sending.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WinningVersion {
    #[serde(rename = "notAvailable")]
    NotAvailable,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "tie")]
    Tie,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
}

impl Default for WinningVersion {
    fn default() -> WinningVersion {
        Self::NotAvailable
    }
}
/// Criteria choosen for winning version (Open/Click)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WinningCriteria {
    #[serde(rename = "Open")]
    Open,
    #[serde(rename = "Click")]
    Click,
}

impl Default for WinningCriteria {
    fn default() -> WinningCriteria {
        Self::Open
    }
}
