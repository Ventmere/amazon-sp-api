# Rust API client for amazon-sp-catalog-items

The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v0
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-catalog-items` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-catalog-items = { path = "./amazon-sp-catalog-items" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CatalogApi* | [**get_catalog_item**](docs/CatalogApi.md#get_catalog_item) | **GET** /catalog/v0/items/{asin} | 
*CatalogApi* | [**list_catalog_categories**](docs/CatalogApi.md#list_catalog_categories) | **GET** /catalog/v0/categories | 
*CatalogApi* | [**list_catalog_items**](docs/CatalogApi.md#list_catalog_items) | **GET** /catalog/v0/items | 


## Documentation For Models

 - [AsinIdentifier](docs/AsinIdentifier.md)
 - [AttributeSetListType](docs/AttributeSetListType.md)
 - [Categories](docs/Categories.md)
 - [CreatorType](docs/CreatorType.md)
 - [DecimalWithUnits](docs/DecimalWithUnits.md)
 - [DimensionType](docs/DimensionType.md)
 - [Error](docs/Error.md)
 - [GetCatalogItemResponse](docs/GetCatalogItemResponse.md)
 - [IdentifierType](docs/IdentifierType.md)
 - [Image](docs/Image.md)
 - [Item](docs/Item.md)
 - [LanguageType](docs/LanguageType.md)
 - [ListCatalogCategoriesResponse](docs/ListCatalogCategoriesResponse.md)
 - [ListCatalogItemsResponse](docs/ListCatalogItemsResponse.md)
 - [ListMatchingItemsResponse](docs/ListMatchingItemsResponse.md)
 - [Price](docs/Price.md)
 - [RelationshipType](docs/RelationshipType.md)
 - [SalesRankType](docs/SalesRankType.md)
 - [SellerSkuIdentifier](docs/SellerSkuIdentifier.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


