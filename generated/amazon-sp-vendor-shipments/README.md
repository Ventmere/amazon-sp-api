# Rust API client for amazon-sp-vendor-shipments

The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-vendor-shipments` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-vendor-shipments = { path = "./amazon-sp-vendor-shipments" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*VendorShippingApi* | [**submit_shipment_confirmations**](docs/VendorShippingApi.md#submit_shipment_confirmations) | **POST** /vendor/shipping/v1/shipmentConfirmations | 


## Documentation For Models

 - [Address](docs/Address.md)
 - [Carton](docs/Carton.md)
 - [CartonReferenceDetails](docs/CartonReferenceDetails.md)
 - [ContainerIdentification](docs/ContainerIdentification.md)
 - [ContainerItem](docs/ContainerItem.md)
 - [Dimensions](docs/Dimensions.md)
 - [Duration](docs/Duration.md)
 - [Error](docs/Error.md)
 - [Expiry](docs/Expiry.md)
 - [ImportDetails](docs/ImportDetails.md)
 - [Item](docs/Item.md)
 - [ItemDetails](docs/ItemDetails.md)
 - [ItemQuantity](docs/ItemQuantity.md)
 - [Location](docs/Location.md)
 - [Money](docs/Money.md)
 - [Pallet](docs/Pallet.md)
 - [PartyIdentification](docs/PartyIdentification.md)
 - [Route](docs/Route.md)
 - [ShipmentConfirmation](docs/ShipmentConfirmation.md)
 - [ShipmentMeasurements](docs/ShipmentMeasurements.md)
 - [Stop](docs/Stop.md)
 - [SubmitShipmentConfirmationsRequest](docs/SubmitShipmentConfirmationsRequest.md)
 - [SubmitShipmentConfirmationsResponse](docs/SubmitShipmentConfirmationsResponse.md)
 - [TaxRegistrationDetails](docs/TaxRegistrationDetails.md)
 - [TransactionReference](docs/TransactionReference.md)
 - [TransportationDetails](docs/TransportationDetails.md)
 - [Volume](docs/Volume.md)
 - [Weight](docs/Weight.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


