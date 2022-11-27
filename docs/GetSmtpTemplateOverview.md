# GetSmtpTemplateOverview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | ID of the template | 
**name** | **String** | Name of the template | 
**subject** | **String** | Subject of the template | 
**is_active** | **bool** | Status of template (true=active, false=inactive) | 
**test_sent** | **bool** | Status of test sending for the template (true=test email has been sent, false=test email has not been sent) | 
**sender** | [**crate::models::GetSmtpTemplateOverviewSender**](getSmtpTemplateOverview_sender.md) |  | 
**reply_to** | **String** | Email defined as the \"Reply to\" for the template | 
**to_field** | **String** | Customisation of the \"to\" field for the template | 
**tag** | **String** | Tag of the template | 
**html_content** | **String** | HTML content of the template | 
**created_at** | **String** | Creation UTC date-time of the template (YYYY-MM-DDTHH:mm:ss.SSSZ) | 
**modified_at** | **String** | Last modification UTC date-time of the template (YYYY-MM-DDTHH:mm:ss.SSSZ) | 
**doi_template** | Option<**bool**> | It is true if template is a valid Double opt-in (DOI) template, otherwise it is false. This field will be available only in case of single template detail call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


