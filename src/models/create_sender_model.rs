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
pub struct CreateSenderModel {
    /// ID of the Sender created
    #[serde(rename = "id")]
    pub id: i64,
    /// Status of SPF configuration for the sender (true = SPF not well configured, false = SPF well configured)
    #[serde(rename = "spfError", skip_serializing_if = "Option::is_none")]
    pub spf_error: Option<bool>,
    /// Status of DKIM configuration for the sender (true = DKIM not well configured, false = DKIM well configured)
    #[serde(rename = "dkimError", skip_serializing_if = "Option::is_none")]
    pub dkim_error: Option<bool>,
}

impl CreateSenderModel {
    pub fn new(id: i64) -> CreateSenderModel {
        CreateSenderModel {
            id,
            spf_error: None,
            dkim_error: None,
        }
    }
}


