# \ResellerApi

All URIs are relative to *https://api.sendinblue.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_credits**](ResellerApi.md#add_credits) | **POST** /reseller/children/{childIdentifier}/credits/add | Add Email and/or SMS credits to a specific child account
[**associate_ip_to_child**](ResellerApi.md#associate_ip_to_child) | **POST** /reseller/children/{childIdentifier}/ips/associate | Associate a dedicated IP to the child
[**create_child_domain**](ResellerApi.md#create_child_domain) | **POST** /reseller/children/{childIdentifier}/domains | Create a domain for a child account
[**create_reseller_child**](ResellerApi.md#create_reseller_child) | **POST** /reseller/children | Creates a reseller child
[**delete_child_domain**](ResellerApi.md#delete_child_domain) | **DELETE** /reseller/children/{childIdentifier}/domains/{domainName} | Delete the sender domain of the reseller child based on the childIdentifier and domainName passed
[**delete_reseller_child**](ResellerApi.md#delete_reseller_child) | **DELETE** /reseller/children/{childIdentifier} | Delete a single reseller child based on the child identifier supplied
[**dissociate_ip_from_child**](ResellerApi.md#dissociate_ip_from_child) | **POST** /reseller/children/{childIdentifier}/ips/dissociate | Dissociate a dedicated IP to the child
[**get_child_account_creation_status**](ResellerApi.md#get_child_account_creation_status) | **GET** /reseller/children/{childIdentifier}/accountCreationStatus | Get the status of a reseller's child account creation, whether it is successfully created (exists) or not based on the childIdentifier supplied
[**get_child_domains**](ResellerApi.md#get_child_domains) | **GET** /reseller/children/{childIdentifier}/domains | Get all sender domains for a specific child account
[**get_child_info**](ResellerApi.md#get_child_info) | **GET** /reseller/children/{childIdentifier} | Get a child account's details
[**get_reseller_childs**](ResellerApi.md#get_reseller_childs) | **GET** /reseller/children | Get the list of all children accounts
[**get_sso_token**](ResellerApi.md#get_sso_token) | **GET** /reseller/children/{childIdentifier}/auth | Get session token to access Sendinblue (SSO)
[**remove_credits**](ResellerApi.md#remove_credits) | **POST** /reseller/children/{childIdentifier}/credits/remove | Remove Email and/or SMS credits from a specific child account
[**update_child_account_status**](ResellerApi.md#update_child_account_status) | **PUT** /reseller/children/{childIdentifier}/accountStatus | Update info of reseller's child account status based on the identifier supplied
[**update_child_domain**](ResellerApi.md#update_child_domain) | **PUT** /reseller/children/{childIdentifier}/domains/{domainName} | Update the sender domain of reseller's child based on the childIdentifier and domainName passed
[**update_reseller_child**](ResellerApi.md#update_reseller_child) | **PUT** /reseller/children/{childIdentifier} | Update info of reseller's child based on the child identifier supplied



## add_credits

> crate::models::RemainingCreditModel add_credits(child_identifier, add_credits)
Add Email and/or SMS credits to a specific child account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |
**add_credits** | [**AddCredits**](AddCredits.md) | Values to post to add credit to a specific child account | [required] |

### Return type

[**crate::models::RemainingCreditModel**](remainingCreditModel.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## associate_ip_to_child

> associate_ip_to_child(child_identifier, ip)
Associate a dedicated IP to the child

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |
**ip** | [**ManageIp**](ManageIp.md) | IP to associate | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_child_domain

> create_child_domain(child_identifier, add_child_domain)
Create a domain for a child account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |
**add_child_domain** | [**AddChildDomain**](AddChildDomain.md) | Sender domain to add for a specific child account. This will not be displayed to the parent account. | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_reseller_child

> crate::models::CreateReseller create_reseller_child(reseller_child)
Creates a reseller child

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reseller_child** | Option<[**CreateChild**](CreateChild.md)> | reseller child to add |  |

### Return type

[**crate::models::CreateReseller**](createReseller.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_child_domain

> delete_child_domain(child_identifier, domain_name)
Delete the sender domain of the reseller child based on the childIdentifier and domainName passed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |
**domain_name** | **String** | Pass the existing domain that needs to be deleted | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_reseller_child

> delete_reseller_child(child_identifier)
Delete a single reseller child based on the child identifier supplied

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or child id of reseller's child | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dissociate_ip_from_child

> dissociate_ip_from_child(child_identifier, ip)
Dissociate a dedicated IP to the child

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |
**ip** | [**ManageIp**](ManageIp.md) | IP to dissociate | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_child_account_creation_status

> crate::models::GetChildAccountCreationStatus get_child_account_creation_status(child_identifier)
Get the status of a reseller's child account creation, whether it is successfully created (exists) or not based on the childIdentifier supplied

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |

### Return type

[**crate::models::GetChildAccountCreationStatus**](getChildAccountCreationStatus.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_child_domains

> Vec<crate::models::GetChildDomainsInner> get_child_domains(child_identifier)
Get all sender domains for a specific child account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |

### Return type

[**Vec<crate::models::GetChildDomainsInner>**](getChildDomains_inner.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_child_info

> crate::models::GetChildInfo get_child_info(child_identifier)
Get a child account's details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |

### Return type

[**crate::models::GetChildInfo**](getChildInfo.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reseller_childs

> crate::models::GetChildrenList get_reseller_childs(limit, offset)
Get the list of all children accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of documents for child accounts information per page |  |[default to 10]
**offset** | Option<**i64**> | Index of the first document in the page |  |[default to 0]

### Return type

[**crate::models::GetChildrenList**](getChildrenList.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sso_token

> crate::models::GetSsoToken get_sso_token(child_identifier)
Get session token to access Sendinblue (SSO)

It returns a session [token] which will remain valid for a short period of time. A child account will be able to access a white-labeled section by using the following url pattern => https:/email.mydomain.com/login/sso?token=[token]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |

### Return type

[**crate::models::GetSsoToken**](getSsoToken.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_credits

> crate::models::RemainingCreditModel remove_credits(child_identifier, remove_credits)
Remove Email and/or SMS credits from a specific child account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |
**remove_credits** | [**RemoveCredits**](RemoveCredits.md) | Values to post to remove email or SMS credits from a specific child account | [required] |

### Return type

[**crate::models::RemainingCreditModel**](remainingCreditModel.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_child_account_status

> update_child_account_status(child_identifier, update_child_account_status)
Update info of reseller's child account status based on the identifier supplied

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |
**update_child_account_status** | [**UpdateChildAccountStatus**](UpdateChildAccountStatus.md) | values to update in child account status | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_child_domain

> update_child_domain(child_identifier, domain_name, update_child_domain)
Update the sender domain of reseller's child based on the childIdentifier and domainName passed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |
**domain_name** | **String** | Pass the existing domain that needs to be updated | [required] |
**update_child_domain** | [**UpdateChildDomain**](UpdateChildDomain.md) | value to update for sender domain | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_reseller_child

> update_reseller_child(child_identifier, reseller_child)
Update info of reseller's child based on the child identifier supplied

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_identifier** | **String** | Either auth key or id of reseller's child | [required] |
**reseller_child** | [**UpdateChild**](UpdateChild.md) | values to update in child profile | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

