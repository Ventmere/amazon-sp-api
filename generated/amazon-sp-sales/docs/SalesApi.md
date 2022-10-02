# \SalesApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_order_metrics**](SalesApi.md#get_order_metrics) | **GET** /sales/v1/orderMetrics | 



## get_order_metrics

> crate::models::GetOrderMetricsResponse get_order_metrics(marketplace_ids, interval, granularity, granularity_time_zone, buyer_type, fulfillment_network, first_day_of_week, asin, sku)


Returns aggregated order metrics for given interval, broken down by granularity, for given buyer type.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | .5 | 15 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified.  For example, ATVPDKIKX0DER indicates the US marketplace. | [required] |
**interval** | **String** | A time interval used for selecting order metrics. This takes the form of two dates separated by two hyphens (first date is inclusive; second date is exclusive). Dates are in ISO8601 format and must represent absolute time (either Z notation or offset notation). Example: 2018-09-01T00:00:00-07:00--2018-09-04T00:00:00-07:00 requests order metrics for Sept 1st, 2nd and 3rd in the -07:00 zone. | [required] |
**granularity** | **String** | The granularity of the grouping of order metrics, based on a unit of time. Specifying granularity=Hour results in a successful request only if the interval specified is less than or equal to 30 days from now. For all other granularities, the interval specified must be less or equal to 2 years from now. Specifying granularity=Total results in order metrics that are aggregated over the entire interval that you specify. If the interval start and end date don’t align with the specified granularity, the head and tail end of the response interval will contain partial data. Example: Day to get a daily breakdown of the request interval, where the day boundary is defined by the granularityTimeZone. | [required] |
**granularity_time_zone** | Option<**String**> | An IANA-compatible time zone for determining the day boundary. Required when specifying a granularity value greater than Hour. The granularityTimeZone value must align with the offset of the specified interval value. For example, if the interval value uses Z notation, then granularityTimeZone must be UTC. If the interval value uses an offset, then granularityTimeZone must be an IANA-compatible time zone that matches the offset. Example: US/Pacific to compute day boundaries, accounting for daylight time savings, for US/Pacific zone. |  |
**buyer_type** | Option<**String**> | Filters the results by the buyer type that you specify, B2B (business to business) or B2C (business to customer). Example: B2B, if you want the response to include order metrics for only B2B buyers. |  |[default to All]
**fulfillment_network** | Option<**String**> | Filters the results by the fulfillment network that you specify, MFN (merchant fulfillment network) or AFN (Amazon fulfillment network). Do not include this filter if you want the response to include order metrics for all fulfillment networks. Example: AFN, if you want the response to include order metrics for only Amazon fulfillment network. |  |
**first_day_of_week** | Option<**String**> | Specifies the day that the week starts on when granularity=Week, either Monday or Sunday. Default: Monday. Example: Sunday, if you want the week to start on a Sunday. |  |[default to Monday]
**asin** | Option<**String**> | Filters the results by the ASIN that you specify. Specifying both ASIN and SKU returns an error. Do not include this filter if you want the response to include order metrics for all ASINs. Example: B0792R1RSN, if you want the response to include order metrics for only ASIN B0792R1RSN. |  |
**sku** | Option<**String**> | Filters the results by the SKU that you specify. Specifying both ASIN and SKU returns an error. Do not include this filter if you want the response to include order metrics for all SKUs. Example: TestSKU, if you want the response to include order metrics for only SKU TestSKU. |  |

### Return type

[**crate::models::GetOrderMetricsResponse**](GetOrderMetricsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, payload

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

