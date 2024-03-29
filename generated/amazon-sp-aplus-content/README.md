# Rust API client for amazon-sp-aplus-content

With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2020-11-01
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-aplus-content` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-aplus-content = { path = "./amazon-sp-aplus-content" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AplusContentApi* | [**create_content_document**](docs/AplusContentApi.md#create_content_document) | **POST** /aplus/2020-11-01/contentDocuments | 
*AplusContentApi* | [**get_content_document**](docs/AplusContentApi.md#get_content_document) | **GET** /aplus/2020-11-01/contentDocuments/{contentReferenceKey} | 
*AplusContentApi* | [**list_content_document_asin_relations**](docs/AplusContentApi.md#list_content_document_asin_relations) | **GET** /aplus/2020-11-01/contentDocuments/{contentReferenceKey}/asins | 
*AplusContentApi* | [**post_content_document_approval_submission**](docs/AplusContentApi.md#post_content_document_approval_submission) | **POST** /aplus/2020-11-01/contentDocuments/{contentReferenceKey}/approvalSubmissions | 
*AplusContentApi* | [**post_content_document_asin_relations**](docs/AplusContentApi.md#post_content_document_asin_relations) | **POST** /aplus/2020-11-01/contentDocuments/{contentReferenceKey}/asins | 
*AplusContentApi* | [**post_content_document_suspend_submission**](docs/AplusContentApi.md#post_content_document_suspend_submission) | **POST** /aplus/2020-11-01/contentDocuments/{contentReferenceKey}/suspendSubmissions | 
*AplusContentApi* | [**search_content_documents**](docs/AplusContentApi.md#search_content_documents) | **GET** /aplus/2020-11-01/contentDocuments | 
*AplusContentApi* | [**search_content_publish_records**](docs/AplusContentApi.md#search_content_publish_records) | **GET** /aplus/2020-11-01/contentPublishRecords | 
*AplusContentApi* | [**update_content_document**](docs/AplusContentApi.md#update_content_document) | **POST** /aplus/2020-11-01/contentDocuments/{contentReferenceKey} | 
*AplusContentApi* | [**validate_content_document_asin_relations**](docs/AplusContentApi.md#validate_content_document_asin_relations) | **POST** /aplus/2020-11-01/contentAsinValidations | 


## Documentation For Models

 - [AplusPaginatedResponse](docs/AplusPaginatedResponse.md)
 - [AplusPaginatedResponseAllOf](docs/AplusPaginatedResponseAllOf.md)
 - [AplusResponse](docs/AplusResponse.md)
 - [AsinBadge](docs/AsinBadge.md)
 - [AsinMetadata](docs/AsinMetadata.md)
 - [ColorType](docs/ColorType.md)
 - [ContentBadge](docs/ContentBadge.md)
 - [ContentDocument](docs/ContentDocument.md)
 - [ContentMetadata](docs/ContentMetadata.md)
 - [ContentMetadataRecord](docs/ContentMetadataRecord.md)
 - [ContentModule](docs/ContentModule.md)
 - [ContentModuleType](docs/ContentModuleType.md)
 - [ContentRecord](docs/ContentRecord.md)
 - [ContentStatus](docs/ContentStatus.md)
 - [ContentType](docs/ContentType.md)
 - [Decorator](docs/Decorator.md)
 - [DecoratorType](docs/DecoratorType.md)
 - [Error](docs/Error.md)
 - [ErrorList](docs/ErrorList.md)
 - [GetContentDocumentResponse](docs/GetContentDocumentResponse.md)
 - [GetContentDocumentResponseAllOf](docs/GetContentDocumentResponseAllOf.md)
 - [ImageComponent](docs/ImageComponent.md)
 - [ImageCropSpecification](docs/ImageCropSpecification.md)
 - [ImageDimensions](docs/ImageDimensions.md)
 - [ImageOffsets](docs/ImageOffsets.md)
 - [IntegerWithUnits](docs/IntegerWithUnits.md)
 - [ListContentDocumentAsinRelationsResponse](docs/ListContentDocumentAsinRelationsResponse.md)
 - [ListContentDocumentAsinRelationsResponseAllOf](docs/ListContentDocumentAsinRelationsResponseAllOf.md)
 - [ParagraphComponent](docs/ParagraphComponent.md)
 - [PlainTextItem](docs/PlainTextItem.md)
 - [PositionType](docs/PositionType.md)
 - [PostContentDocumentApprovalSubmissionResponse](docs/PostContentDocumentApprovalSubmissionResponse.md)
 - [PostContentDocumentAsinRelationsRequest](docs/PostContentDocumentAsinRelationsRequest.md)
 - [PostContentDocumentAsinRelationsResponse](docs/PostContentDocumentAsinRelationsResponse.md)
 - [PostContentDocumentRequest](docs/PostContentDocumentRequest.md)
 - [PostContentDocumentResponse](docs/PostContentDocumentResponse.md)
 - [PostContentDocumentResponseAllOf](docs/PostContentDocumentResponseAllOf.md)
 - [PostContentDocumentSuspendSubmissionResponse](docs/PostContentDocumentSuspendSubmissionResponse.md)
 - [PublishRecord](docs/PublishRecord.md)
 - [SearchContentDocumentsResponse](docs/SearchContentDocumentsResponse.md)
 - [SearchContentDocumentsResponseAllOf](docs/SearchContentDocumentsResponseAllOf.md)
 - [SearchContentPublishRecordsResponse](docs/SearchContentPublishRecordsResponse.md)
 - [SearchContentPublishRecordsResponseAllOf](docs/SearchContentPublishRecordsResponseAllOf.md)
 - [StandardCompanyLogoModule](docs/StandardCompanyLogoModule.md)
 - [StandardComparisonProductBlock](docs/StandardComparisonProductBlock.md)
 - [StandardComparisonTableModule](docs/StandardComparisonTableModule.md)
 - [StandardFourImageTextModule](docs/StandardFourImageTextModule.md)
 - [StandardFourImageTextQuadrantModule](docs/StandardFourImageTextQuadrantModule.md)
 - [StandardHeaderImageTextModule](docs/StandardHeaderImageTextModule.md)
 - [StandardHeaderTextListBlock](docs/StandardHeaderTextListBlock.md)
 - [StandardImageCaptionBlock](docs/StandardImageCaptionBlock.md)
 - [StandardImageSidebarModule](docs/StandardImageSidebarModule.md)
 - [StandardImageTextBlock](docs/StandardImageTextBlock.md)
 - [StandardImageTextCaptionBlock](docs/StandardImageTextCaptionBlock.md)
 - [StandardImageTextOverlayModule](docs/StandardImageTextOverlayModule.md)
 - [StandardMultipleImageTextModule](docs/StandardMultipleImageTextModule.md)
 - [StandardProductDescriptionModule](docs/StandardProductDescriptionModule.md)
 - [StandardSingleImageHighlightsModule](docs/StandardSingleImageHighlightsModule.md)
 - [StandardSingleImageSpecsDetailModule](docs/StandardSingleImageSpecsDetailModule.md)
 - [StandardSingleSideImageModule](docs/StandardSingleSideImageModule.md)
 - [StandardTechSpecsModule](docs/StandardTechSpecsModule.md)
 - [StandardTextBlock](docs/StandardTextBlock.md)
 - [StandardTextListBlock](docs/StandardTextListBlock.md)
 - [StandardTextModule](docs/StandardTextModule.md)
 - [StandardTextPairBlock](docs/StandardTextPairBlock.md)
 - [StandardThreeImageTextModule](docs/StandardThreeImageTextModule.md)
 - [TextComponent](docs/TextComponent.md)
 - [TextItem](docs/TextItem.md)
 - [ValidateContentDocumentAsinRelationsResponse](docs/ValidateContentDocumentAsinRelationsResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



