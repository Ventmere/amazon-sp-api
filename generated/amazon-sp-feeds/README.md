# Rust API client for amazon-sp-feeds

The Selling Partner API for Feeds lets you upload data to Amazon on behalf of a selling partner.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2020-09-04
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-feeds` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-feeds = { path = "./amazon-sp-feeds" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*FeedsApi* | [**cancel_feed**](docs/FeedsApi.md#cancel_feed) | **DELETE** /feeds/2020-09-04/feeds/{feedId} | 
*FeedsApi* | [**create_feed**](docs/FeedsApi.md#create_feed) | **POST** /feeds/2020-09-04/feeds | 
*FeedsApi* | [**create_feed_document**](docs/FeedsApi.md#create_feed_document) | **POST** /feeds/2020-09-04/documents | 
*FeedsApi* | [**get_feed**](docs/FeedsApi.md#get_feed) | **GET** /feeds/2020-09-04/feeds/{feedId} | 
*FeedsApi* | [**get_feed_document**](docs/FeedsApi.md#get_feed_document) | **GET** /feeds/2020-09-04/documents/{feedDocumentId} | 
*FeedsApi* | [**get_feeds**](docs/FeedsApi.md#get_feeds) | **GET** /feeds/2020-09-04/feeds | 


## Documentation For Models

 - [CancelFeedResponse](docs/CancelFeedResponse.md)
 - [CreateFeedDocumentResponse](docs/CreateFeedDocumentResponse.md)
 - [CreateFeedDocumentResult](docs/CreateFeedDocumentResult.md)
 - [CreateFeedDocumentSpecification](docs/CreateFeedDocumentSpecification.md)
 - [CreateFeedResponse](docs/CreateFeedResponse.md)
 - [CreateFeedResult](docs/CreateFeedResult.md)
 - [CreateFeedSpecification](docs/CreateFeedSpecification.md)
 - [Error](docs/Error.md)
 - [Feed](docs/Feed.md)
 - [FeedDocument](docs/FeedDocument.md)
 - [FeedDocumentEncryptionDetails](docs/FeedDocumentEncryptionDetails.md)
 - [GetFeedDocumentResponse](docs/GetFeedDocumentResponse.md)
 - [GetFeedResponse](docs/GetFeedResponse.md)
 - [GetFeedsResponse](docs/GetFeedsResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



