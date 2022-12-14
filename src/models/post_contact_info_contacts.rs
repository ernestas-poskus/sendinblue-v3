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
pub struct PostContactInfoContacts {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<Vec<String>>,
    #[serde(rename = "failure", skip_serializing_if = "Option::is_none")]
    pub failure: Option<Vec<String>>,
    /// Displays the count of total number of contacts removed from list when user opts for \"all\" option.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Id of the process created to remove contacts from list when user opts for \"all\" option.
    #[serde(rename = "processId", skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i64>,
}

impl PostContactInfoContacts {
    pub fn new() -> PostContactInfoContacts {
        PostContactInfoContacts {
            success: None,
            failure: None,
            total: None,
            process_id: None,
        }
    }
}


