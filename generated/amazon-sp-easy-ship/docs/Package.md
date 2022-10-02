# Package

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scheduled_package_id** | [**crate::models::ScheduledPackageId**](ScheduledPackageId.md) |  | 
**package_dimensions** | [**crate::models::Dimensions**](Dimensions.md) |  | 
**package_weight** | [**crate::models::Weight**](Weight.md) |  | 
**package_items** | Option<[**Vec<crate::models::Item>**](Item.md)> | A list of items contained in the package. | [optional]
**package_time_slot** | [**crate::models::TimeSlot**](TimeSlot.md) |  | 
**package_identifier** | Option<**String**> | Optional seller-created identifier that is printed on the shipping label to help the seller identify the package. | [optional]
**invoice** | Option<[**crate::models::InvoiceData**](InvoiceData.md)> |  | [optional]
**package_status** | Option<[**crate::models::PackageStatus**](PackageStatus.md)> |  | [optional]
**tracking_details** | Option<[**crate::models::TrackingDetails**](TrackingDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


