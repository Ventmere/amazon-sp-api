# CreateShipmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_request_details** | [**crate::models::ShipmentRequestDetails**](ShipmentRequestDetails.md) |  | 
**shipping_service_id** | **String** | An Amazon-defined shipping service identifier. | 
**shipping_service_offer_id** | Option<**String**> | Identifies a shipping service order made by a carrier. | [optional]
**hazmat_type** | Option<[**crate::models::HazmatType**](HazmatType.md)> |  | [optional]
**label_format_option** | Option<[**crate::models::LabelFormatOptionRequest**](LabelFormatOptionRequest.md)> |  | [optional]
**shipment_level_seller_inputs_list** | Option<[**Vec<crate::models::AdditionalSellerInputs>**](AdditionalSellerInputs.md)> | A list of additional seller input pairs required to purchase shipping. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


