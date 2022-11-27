# \AttributesApi

All URIs are relative to *https://api.sendinblue.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_attribute**](AttributesApi.md#create_attribute) | **POST** /contacts/attributes/{attributeCategory}/{attributeName} | Create contact attribute
[**delete_attribute**](AttributesApi.md#delete_attribute) | **DELETE** /contacts/attributes/{attributeCategory}/{attributeName} | Delete an attribute
[**get_attributes**](AttributesApi.md#get_attributes) | **GET** /contacts/attributes | List all attributes
[**update_attribute**](AttributesApi.md#update_attribute) | **PUT** /contacts/attributes/{attributeCategory}/{attributeName} | Update contact attribute



## create_attribute

> create_attribute(attribute_category, attribute_name, create_attribute)
Create contact attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_category** | **String** | Category of the attribute | [required] |
**attribute_name** | **String** | Name of the attribute | [required] |
**create_attribute** | [**CreateAttribute**](CreateAttribute.md) | Values to create an attribute | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_attribute

> delete_attribute(attribute_category, attribute_name)
Delete an attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_category** | **String** | Category of the attribute | [required] |
**attribute_name** | **String** | Name of the existing attribute | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attributes

> crate::models::GetAttributes get_attributes()
List all attributes

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAttributes**](getAttributes.md)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_attribute

> update_attribute(attribute_category, attribute_name, update_attribute)
Update contact attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_category** | **String** | Category of the attribute | [required] |
**attribute_name** | **String** | Name of the existing attribute | [required] |
**update_attribute** | [**UpdateAttribute**](UpdateAttribute.md) | Values to update an attribute | [required] |

### Return type

 (empty response body)

### Authorization

[api-key](../README.md#api-key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

