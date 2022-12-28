# Rust API client for amazon-sp-vendor-direct-fulfillment-orders

The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-vendor-direct-fulfillment-orders` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-vendor-direct-fulfillment-orders = { path = "./amazon-sp-vendor-direct-fulfillment-orders" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*VendorOrdersApi* | [**get_order**](docs/VendorOrdersApi.md#get_order) | **GET** /vendor/directFulfillment/orders/v1/purchaseOrders/{purchaseOrderNumber} | 
*VendorOrdersApi* | [**get_orders**](docs/VendorOrdersApi.md#get_orders) | **GET** /vendor/directFulfillment/orders/v1/purchaseOrders | 
*VendorOrdersApi* | [**submit_acknowledgement**](docs/VendorOrdersApi.md#submit_acknowledgement) | **POST** /vendor/directFulfillment/orders/v1/acknowledgements | 


## Documentation For Models

 - [AcknowledgementStatus](docs/AcknowledgementStatus.md)
 - [Address](docs/Address.md)
 - [Error](docs/Error.md)
 - [GetOrderResponse](docs/GetOrderResponse.md)
 - [GetOrdersResponse](docs/GetOrdersResponse.md)
 - [GiftDetails](docs/GiftDetails.md)
 - [ItemQuantity](docs/ItemQuantity.md)
 - [Money](docs/Money.md)
 - [Order](docs/Order.md)
 - [OrderAcknowledgementItem](docs/OrderAcknowledgementItem.md)
 - [OrderDetails](docs/OrderDetails.md)
 - [OrderDetailsTaxTotal](docs/OrderDetailsTaxTotal.md)
 - [OrderItem](docs/OrderItem.md)
 - [OrderItemAcknowledgement](docs/OrderItemAcknowledgement.md)
 - [OrderItemTaxDetails](docs/OrderItemTaxDetails.md)
 - [OrderList](docs/OrderList.md)
 - [Pagination](docs/Pagination.md)
 - [PartyIdentification](docs/PartyIdentification.md)
 - [ScheduledDeliveryShipment](docs/ScheduledDeliveryShipment.md)
 - [ShipmentDates](docs/ShipmentDates.md)
 - [ShipmentDetails](docs/ShipmentDetails.md)
 - [SubmitAcknowledgementRequest](docs/SubmitAcknowledgementRequest.md)
 - [SubmitAcknowledgementResponse](docs/SubmitAcknowledgementResponse.md)
 - [TaxDetails](docs/TaxDetails.md)
 - [TaxRegistrationDetails](docs/TaxRegistrationDetails.md)
 - [TransactionId](docs/TransactionId.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


