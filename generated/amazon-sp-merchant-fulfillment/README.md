# Rust API client for amazon-sp-merchant-fulfillment

The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v0
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-merchant-fulfillment` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-merchant-fulfillment = { path = "./amazon-sp-merchant-fulfillment" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*MerchantFulfillmentApi* | [**cancel_shipment**](docs/MerchantFulfillmentApi.md#cancel_shipment) | **DELETE** /mfn/v0/shipments/{shipmentId} | 
*MerchantFulfillmentApi* | [**cancel_shipment_old**](docs/MerchantFulfillmentApi.md#cancel_shipment_old) | **PUT** /mfn/v0/shipments/{shipmentId}/cancel | 
*MerchantFulfillmentApi* | [**create_shipment**](docs/MerchantFulfillmentApi.md#create_shipment) | **POST** /mfn/v0/shipments | 
*MerchantFulfillmentApi* | [**get_additional_seller_inputs**](docs/MerchantFulfillmentApi.md#get_additional_seller_inputs) | **POST** /mfn/v0/additionalSellerInputs | 
*MerchantFulfillmentApi* | [**get_additional_seller_inputs_old**](docs/MerchantFulfillmentApi.md#get_additional_seller_inputs_old) | **POST** /mfn/v0/sellerInputs | 
*MerchantFulfillmentApi* | [**get_eligible_shipment_services**](docs/MerchantFulfillmentApi.md#get_eligible_shipment_services) | **POST** /mfn/v0/eligibleShippingServices | 
*MerchantFulfillmentApi* | [**get_eligible_shipment_services_old**](docs/MerchantFulfillmentApi.md#get_eligible_shipment_services_old) | **POST** /mfn/v0/eligibleServices | 
*MerchantFulfillmentApi* | [**get_shipment**](docs/MerchantFulfillmentApi.md#get_shipment) | **GET** /mfn/v0/shipments/{shipmentId} | 


## Documentation For Models

 - [AdditionalInputs](docs/AdditionalInputs.md)
 - [AdditionalSellerInput](docs/AdditionalSellerInput.md)
 - [AdditionalSellerInputs](docs/AdditionalSellerInputs.md)
 - [Address](docs/Address.md)
 - [AvailableCarrierWillPickUpOption](docs/AvailableCarrierWillPickUpOption.md)
 - [AvailableDeliveryExperienceOption](docs/AvailableDeliveryExperienceOption.md)
 - [AvailableShippingServiceOptions](docs/AvailableShippingServiceOptions.md)
 - [CancelShipmentResponse](docs/CancelShipmentResponse.md)
 - [CarrierWillPickUpOption](docs/CarrierWillPickUpOption.md)
 - [Constraint](docs/Constraint.md)
 - [CreateShipmentRequest](docs/CreateShipmentRequest.md)
 - [CreateShipmentResponse](docs/CreateShipmentResponse.md)
 - [CurrencyAmount](docs/CurrencyAmount.md)
 - [DeliveryExperienceOption](docs/DeliveryExperienceOption.md)
 - [DeliveryExperienceType](docs/DeliveryExperienceType.md)
 - [Error](docs/Error.md)
 - [FileContents](docs/FileContents.md)
 - [FileType](docs/FileType.md)
 - [GetAdditionalSellerInputsRequest](docs/GetAdditionalSellerInputsRequest.md)
 - [GetAdditionalSellerInputsResponse](docs/GetAdditionalSellerInputsResponse.md)
 - [GetAdditionalSellerInputsResult](docs/GetAdditionalSellerInputsResult.md)
 - [GetEligibleShipmentServicesRequest](docs/GetEligibleShipmentServicesRequest.md)
 - [GetEligibleShipmentServicesResponse](docs/GetEligibleShipmentServicesResponse.md)
 - [GetEligibleShipmentServicesResult](docs/GetEligibleShipmentServicesResult.md)
 - [GetShipmentResponse](docs/GetShipmentResponse.md)
 - [HazmatType](docs/HazmatType.md)
 - [InputTargetType](docs/InputTargetType.md)
 - [Item](docs/Item.md)
 - [ItemLevelFields](docs/ItemLevelFields.md)
 - [Label](docs/Label.md)
 - [LabelCustomization](docs/LabelCustomization.md)
 - [LabelDimensions](docs/LabelDimensions.md)
 - [LabelFormat](docs/LabelFormat.md)
 - [LabelFormatOption](docs/LabelFormatOption.md)
 - [LabelFormatOptionRequest](docs/LabelFormatOptionRequest.md)
 - [Length](docs/Length.md)
 - [PackageDimensions](docs/PackageDimensions.md)
 - [PredefinedPackageDimensions](docs/PredefinedPackageDimensions.md)
 - [RejectedShippingService](docs/RejectedShippingService.md)
 - [SellerInputDefinition](docs/SellerInputDefinition.md)
 - [Shipment](docs/Shipment.md)
 - [ShipmentRequestDetails](docs/ShipmentRequestDetails.md)
 - [ShipmentStatus](docs/ShipmentStatus.md)
 - [ShippingOfferingFilter](docs/ShippingOfferingFilter.md)
 - [ShippingService](docs/ShippingService.md)
 - [ShippingServiceOptions](docs/ShippingServiceOptions.md)
 - [StandardIdForLabel](docs/StandardIdForLabel.md)
 - [TemporarilyUnavailableCarrier](docs/TemporarilyUnavailableCarrier.md)
 - [TermsAndConditionsNotAcceptedCarrier](docs/TermsAndConditionsNotAcceptedCarrier.md)
 - [UnitOfLength](docs/UnitOfLength.md)
 - [UnitOfWeight](docs/UnitOfWeight.md)
 - [Weight](docs/Weight.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



