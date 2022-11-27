# GetEmailCampaignsCampaignsInnerAllOfStatistics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**global_stats** | [**crate::models::GetExtendedCampaignStatsGlobalStats**](getExtendedCampaignStats_globalStats.md) |  | 
**campaign_stats** | [**Vec<crate::models::GetExtendedCampaignStatsCampaignStatsInner>**](getExtendedCampaignStats_campaignStats_inner.md) | List-wise statistics of the campaign. | 
**mirror_click** | **i64** | Number of clicks on mirror link | 
**remaining** | **i64** | Number of remaning emails to send | 
**links_stats** | [**serde_json::Value**](.md) | Statistics about the number of clicks for the links | 
**stats_by_domain** | [**::std::collections::HashMap<String, crate::models::GetCampaignStats>**](getCampaignStats.md) |  | 
**stats_by_device** | [**crate::models::GetStatsByDevice**](getStatsByDevice.md) |  | 
**stats_by_browser** | [**::std::collections::HashMap<String, crate::models::GetDeviceBrowserStats>**](getDeviceBrowserStats.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


