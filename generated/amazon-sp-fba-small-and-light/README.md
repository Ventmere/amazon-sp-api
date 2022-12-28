# Rust API client for amazon-sp-fba-small-and-light

The Selling Partner API for FBA Small and Light lets you help sellers manage their listings in the Small and Light program. The program reduces the cost of fulfilling orders for small and lightweight FBA inventory. You can enroll or remove items from the program and check item eligibility and enrollment status. You can also preview the estimated program fees charged to a seller for items sold while enrolled in the program.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-fba-small-and-light` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-fba-small-and-light = { path = "./amazon-sp-fba-small-and-light" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*SmallAndLightApi* | [**delete_small_and_light_enrollment_by_seller_sku**](docs/SmallAndLightApi.md#delete_small_and_light_enrollment_by_seller_sku) | **DELETE** /fba/smallAndLight/v1/enrollments/{sellerSKU} | 
*SmallAndLightApi* | [**get_small_and_light_eligibility_by_seller_sku**](docs/SmallAndLightApi.md#get_small_and_light_eligibility_by_seller_sku) | **GET** /fba/smallAndLight/v1/eligibilities/{sellerSKU} | 
*SmallAndLightApi* | [**get_small_and_light_enrollment_by_seller_sku**](docs/SmallAndLightApi.md#get_small_and_light_enrollment_by_seller_sku) | **GET** /fba/smallAndLight/v1/enrollments/{sellerSKU} | 
*SmallAndLightApi* | [**get_small_and_light_fee_preview**](docs/SmallAndLightApi.md#get_small_and_light_fee_preview) | **POST** /fba/smallAndLight/v1/feePreviews | 
*SmallAndLightApi* | [**put_small_and_light_enrollment_by_seller_sku**](docs/SmallAndLightApi.md#put_small_and_light_enrollment_by_seller_sku) | **PUT** /fba/smallAndLight/v1/enrollments/{sellerSKU} | 


## Documentation For Models

 - [Error](docs/Error.md)
 - [ErrorList](docs/ErrorList.md)
 - [FeeLineItem](docs/FeeLineItem.md)
 - [FeePreview](docs/FeePreview.md)
 - [Item](docs/Item.md)
 - [MoneyType](docs/MoneyType.md)
 - [SmallAndLightEligibility](docs/SmallAndLightEligibility.md)
 - [SmallAndLightEligibilityStatus](docs/SmallAndLightEligibilityStatus.md)
 - [SmallAndLightEnrollment](docs/SmallAndLightEnrollment.md)
 - [SmallAndLightEnrollmentStatus](docs/SmallAndLightEnrollmentStatus.md)
 - [SmallAndLightFeePreviewRequest](docs/SmallAndLightFeePreviewRequest.md)
 - [SmallAndLightFeePreviews](docs/SmallAndLightFeePreviews.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


