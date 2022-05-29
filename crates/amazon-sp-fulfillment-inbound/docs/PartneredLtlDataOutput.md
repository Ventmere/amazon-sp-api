# PartneredLtlDataOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contact** | [**crate::models::Contact**](Contact.md) |  | 
**box_count** | **i64** |  | 
**seller_freight_class** | Option<[**crate::models::SellerFreightClass**](SellerFreightClass.md)> |  | [optional]
**freight_ready_date** | [**String**](string.md) |  | 
**pallet_list** | [**Vec<crate::models::Pallet>**](Pallet.md) | A list of pallet information. | 
**total_weight** | [**crate::models::Weight**](Weight.md) |  | 
**seller_declared_value** | Option<[**crate::models::Amount**](Amount.md)> |  | [optional]
**amazon_calculated_value** | Option<[**crate::models::Amount**](Amount.md)> |  | [optional]
**preview_pickup_date** | [**String**](string.md) |  | 
**preview_delivery_date** | [**String**](string.md) |  | 
**preview_freight_class** | [**crate::models::SellerFreightClass**](SellerFreightClass.md) |  | 
**amazon_reference_id** | **String** | A unique identifier created by Amazon that identifies this Amazon-partnered, Less Than Truckload/Full Truckload (LTL/FTL) shipment. | 
**is_bill_of_lading_available** | **bool** | Indicates whether the bill of lading for the shipment is available. | 
**partnered_estimate** | Option<[**crate::models::PartneredEstimate**](PartneredEstimate.md)> |  | [optional]
**carrier_name** | **String** | The carrier for the inbound shipment. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


