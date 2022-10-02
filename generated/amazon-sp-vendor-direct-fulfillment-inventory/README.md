# Rust API client for amazon-sp-vendor-direct-fulfillment-inventory

The Selling Partner API for Direct Fulfillment Inventory Updates provides programmatic access to a direct fulfillment vendor's inventory updates.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-vendor-direct-fulfillment-inventory` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-vendor-direct-fulfillment-inventory = { path = "./amazon-sp-vendor-direct-fulfillment-inventory" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*UpdateInventoryApi* | [**submit_inventory_update**](docs/UpdateInventoryApi.md#submit_inventory_update) | **POST** /vendor/directFulfillment/inventory/v1/warehouses/{warehouseId}/items | 


## Documentation For Models

 - [Error](docs/Error.md)
 - [InventoryUpdate](docs/InventoryUpdate.md)
 - [ItemDetails](docs/ItemDetails.md)
 - [ItemQuantity](docs/ItemQuantity.md)
 - [PartyIdentification](docs/PartyIdentification.md)
 - [SubmitInventoryUpdateRequest](docs/SubmitInventoryUpdateRequest.md)
 - [SubmitInventoryUpdateResponse](docs/SubmitInventoryUpdateResponse.md)
 - [TransactionReference](docs/TransactionReference.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


