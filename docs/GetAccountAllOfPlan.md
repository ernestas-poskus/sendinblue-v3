# GetAccountAllOfPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Displays the plan type of the user | 
**credits_type** | **String** | This is the type of the credit, \"Send Limit\" is one of the possible types of credit of a user. \"Send Limit\" implies the total number of emails you can send to the subscribers in your account. | 
**credits** | **f32** | Remaining credits of the user | 
**start_date** | Option<[**String**](string.md)> | Date of the period from which the plan will start (only available for \"subscription\" and \"reseller\" plan type) | [optional]
**end_date** | Option<[**String**](string.md)> | Date of the period from which the plan will end (only available for \"subscription\" and \"reseller\" plan type) | [optional]
**user_limit** | Option<**i32**> | Only in case of reseller account. It implies the total number of child accounts you can add to your account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

