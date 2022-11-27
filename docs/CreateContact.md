# CreateContact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> | Email address of the user. Mandatory if \"SMS\" field is not passed in \"attributes\" parameter. Mobile Number in \"SMS\" field should be passed with proper country code. For example {\"SMS\":\"+91xxxxxxxxxx\"} or {\"SMS\":\"0091xxxxxxxxxx\"} | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> | Pass the set of attributes and their values. These attributes must be present in your SendinBlue account. For eg. {\"FNAME\":\"Elly\", \"LNAME\":\"Roger\"} | [optional]
**email_blacklisted** | Option<**bool**> | Set this field to blacklist the contact for emails (emailBlacklisted = true) | [optional]
**sms_blacklisted** | Option<**bool**> | Set this field to blacklist the contact for SMS (smsBlacklisted = true) | [optional]
**list_ids** | Option<**Vec<i64>**> | Ids of the lists to add the contact to | [optional]
**update_enabled** | Option<**bool**> | Facilitate to update the existing contact in the same request (updateEnabled = true) | [optional][default to false]
**smtp_blacklist_sender** | Option<**Vec<String>**> | transactional email forbidden sender for contact. Use only for email Contact ( only available if updateEnabled = true ) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


