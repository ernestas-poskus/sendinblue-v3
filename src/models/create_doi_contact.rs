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
pub struct CreateDoiContact {
    /// Email address where the confirmation email will be sent. This email address will be the identifier for all other contact attributes.
    #[serde(rename = "email")]
    pub email: String,
    /// Pass the set of attributes and their values. These attributes must be present in your SendinBlue account. For eg. {'FNAME':'Elly', 'LNAME':'Roger'}
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    /// Lists under user account where contact should be added
    #[serde(rename = "includeListIds")]
    pub include_list_ids: Vec<i64>,
    /// Lists under user account where contact should not be added
    #[serde(rename = "excludeListIds", skip_serializing_if = "Option::is_none")]
    pub exclude_list_ids: Option<Vec<i64>>,
    /// Id of the Double opt-in (DOI) template
    #[serde(rename = "templateId")]
    pub template_id: i64,
    /// URL of the web page that user will be redirected to after clicking on the double opt in URL. When editing your DOI template you can reference this URL by using the tag {{ params.DOIurl }}.
    #[serde(rename = "redirectionUrl")]
    pub redirection_url: String,
}

impl CreateDoiContact {
    pub fn new(email: String, include_list_ids: Vec<i64>, template_id: i64, redirection_url: String) -> CreateDoiContact {
        CreateDoiContact {
            email,
            attributes: None,
            include_list_ids,
            exclude_list_ids: None,
            template_id,
            redirection_url,
        }
    }
}


