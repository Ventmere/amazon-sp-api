# Rust API client for amazon-sp-listings-items

The Selling Partner API for Listings Items (Listings Items API) provides programmatic access to selling partner listings on Amazon. Use this API in collaboration with the Selling Partner API for Product Type Definitions, which you use to retrieve the information about Amazon product types needed to use the Listings Items API.

For more information, see the [Listing Items API Use Case Guide](doc:listings-items-api-v2020-09-01-use-case-guide).

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2020-09-01
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-listings-items` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-listings-items = { path = "./amazon-sp-listings-items" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ListingsApi* | [**delete_listings_item**](docs/ListingsApi.md#delete_listings_item) | **DELETE** /listings/2020-09-01/items/{sellerId}/{sku} | 
*ListingsApi* | [**patch_listings_item**](docs/ListingsApi.md#patch_listings_item) | **PATCH** /listings/2020-09-01/items/{sellerId}/{sku} | 
*ListingsApi* | [**put_listings_item**](docs/ListingsApi.md#put_listings_item) | **PUT** /listings/2020-09-01/items/{sellerId}/{sku} | 


## Documentation For Models

 - [Error](docs/Error.md)
 - [ErrorList](docs/ErrorList.md)
 - [Issue](docs/Issue.md)
 - [ListingsItemPatchRequest](docs/ListingsItemPatchRequest.md)
 - [ListingsItemPutRequest](docs/ListingsItemPutRequest.md)
 - [ListingsItemSubmissionResponse](docs/ListingsItemSubmissionResponse.md)
 - [PatchOperation](docs/PatchOperation.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



