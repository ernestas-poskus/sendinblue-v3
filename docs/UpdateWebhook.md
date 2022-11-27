# UpdateWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> | URL of the webhook | [optional]
**description** | Option<**String**> | Description of the webhook | [optional]
**events** | Option<**Vec<String>**> | Events triggering the webhook. Possible values for Transactional type webhook – `sent` OR `request`, `delivered`, `hardBounce`, `softBounce`, `blocked`, `spam`, `invalid`, `deferred`, `click`, `opened`, `uniqueOpened` and `unsubscribed` and possible values for Marketing type webhook – `spam`, `opened`, `click`, `hardBounce`, `softBounce`, `unsubscribed`, `listAddition` and `delivered` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


