# GetContactDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Email address of the contact for which you requested the details | 
**id** | **i64** | ID of the contact for which you requested the details | 
**email_blacklisted** | **bool** | Blacklist status for email campaigns (true=blacklisted, false=not blacklisted) | 
**sms_blacklisted** | **bool** | Blacklist status for SMS campaigns (true=blacklisted, false=not blacklisted) | 
**created_at** | **String** | Creation UTC date-time of the contact (YYYY-MM-DDTHH:mm:ss.SSSZ) | 
**modified_at** | **String** | Last modification UTC date-time of the contact (YYYY-MM-DDTHH:mm:ss.SSSZ) | 
**list_ids** | **Vec<i64>** |  | 
**list_unsubscribed** | Option<**Vec<i64>**> |  | [optional]
**attributes** | [**serde_json::Value**](.md) | Set of attributes of the contact | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


