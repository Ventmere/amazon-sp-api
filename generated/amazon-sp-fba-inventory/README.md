# Rust API client for amazon-sp-fba-inventory

The Selling Partner API for FBA Inventory lets you programmatically retrieve information about inventory in Amazon's fulfillment network.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-fba-inventory` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-fba-inventory = { path = "./amazon-sp-fba-inventory" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*FbaInventoryApi* | [**get_inventory_summaries**](docs/FbaInventoryApi.md#get_inventory_summaries) | **GET** /fba/inventory/v1/summaries | 


## Documentation For Models

 - [Error](docs/Error.md)
 - [GetInventorySummariesResponse](docs/GetInventorySummariesResponse.md)
 - [GetInventorySummariesResult](docs/GetInventorySummariesResult.md)
 - [Granularity](docs/Granularity.md)
 - [InventoryDetails](docs/InventoryDetails.md)
 - [InventorySummary](docs/InventorySummary.md)
 - [Pagination](docs/Pagination.md)
 - [ResearchingQuantity](docs/ResearchingQuantity.md)
 - [ResearchingQuantityEntry](docs/ResearchingQuantityEntry.md)
 - [ReservedQuantity](docs/ReservedQuantity.md)
 - [UnfulfillableQuantity](docs/UnfulfillableQuantity.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


