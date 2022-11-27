# CreateSenderIpsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ip** | **String** | Dedicated IP available in your account | 
**domain** | **String** | Domain of the IP | 
**weight** | Option<**i64**> | Weight to apply to the IP. Sum of all IP weights must be 100. Should be passed for either ALL or NONE of the IPs. If it's not passed, the sending will be equally balanced on all IPs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


