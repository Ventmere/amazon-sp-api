# Rust API client for amazon-sp-uploads

The Uploads API lets you upload files that you can programmatically access using other Selling Partner APIs, such as the A+ Content API and the Messaging API.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2020-11-01
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-uploads` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-uploads = { path = "./amazon-sp-uploads" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*UploadsApi* | [**create_upload_destination_for_resource**](docs/UploadsApi.md#create_upload_destination_for_resource) | **POST** /uploads/2020-11-01/uploadDestinations/{resource} | 


## Documentation For Models

 - [CreateUploadDestinationResponse](docs/CreateUploadDestinationResponse.md)
 - [Error](docs/Error.md)
 - [UploadDestination](docs/UploadDestination.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


