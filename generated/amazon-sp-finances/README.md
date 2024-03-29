# Rust API client for amazon-sp-finances

The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v0
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-finances` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-finances = { path = "./amazon-sp-finances" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**list_financial_event_groups**](docs/DefaultApi.md#list_financial_event_groups) | **GET** /finances/v0/financialEventGroups | 
*DefaultApi* | [**list_financial_events**](docs/DefaultApi.md#list_financial_events) | **GET** /finances/v0/financialEvents | 
*DefaultApi* | [**list_financial_events_by_group_id**](docs/DefaultApi.md#list_financial_events_by_group_id) | **GET** /finances/v0/financialEventGroups/{eventGroupId}/financialEvents | 
*DefaultApi* | [**list_financial_events_by_order_id**](docs/DefaultApi.md#list_financial_events_by_order_id) | **GET** /finances/v0/orders/{orderId}/financialEvents | 


## Documentation For Models

 - [AdhocDisbursementEvent](docs/AdhocDisbursementEvent.md)
 - [AdjustmentEvent](docs/AdjustmentEvent.md)
 - [AdjustmentItem](docs/AdjustmentItem.md)
 - [AffordabilityExpenseEvent](docs/AffordabilityExpenseEvent.md)
 - [CapacityReservationBillingEvent](docs/CapacityReservationBillingEvent.md)
 - [ChargeComponent](docs/ChargeComponent.md)
 - [ChargeInstrument](docs/ChargeInstrument.md)
 - [ChargeRefundEvent](docs/ChargeRefundEvent.md)
 - [ChargeRefundTransaction](docs/ChargeRefundTransaction.md)
 - [CouponPaymentEvent](docs/CouponPaymentEvent.md)
 - [Currency](docs/Currency.md)
 - [DebtRecoveryEvent](docs/DebtRecoveryEvent.md)
 - [DebtRecoveryItem](docs/DebtRecoveryItem.md)
 - [DirectPayment](docs/DirectPayment.md)
 - [Error](docs/Error.md)
 - [FailedAdhocDisbursementEventList](docs/FailedAdhocDisbursementEventList.md)
 - [FbaLiquidationEvent](docs/FbaLiquidationEvent.md)
 - [FeeComponent](docs/FeeComponent.md)
 - [FinancialEventGroup](docs/FinancialEventGroup.md)
 - [FinancialEvents](docs/FinancialEvents.md)
 - [ImagingServicesFeeEvent](docs/ImagingServicesFeeEvent.md)
 - [ListFinancialEventGroupsPayload](docs/ListFinancialEventGroupsPayload.md)
 - [ListFinancialEventGroupsResponse](docs/ListFinancialEventGroupsResponse.md)
 - [ListFinancialEventsPayload](docs/ListFinancialEventsPayload.md)
 - [ListFinancialEventsResponse](docs/ListFinancialEventsResponse.md)
 - [LoanServicingEvent](docs/LoanServicingEvent.md)
 - [NetworkComminglingTransactionEvent](docs/NetworkComminglingTransactionEvent.md)
 - [PayWithAmazonEvent](docs/PayWithAmazonEvent.md)
 - [ProductAdsPaymentEvent](docs/ProductAdsPaymentEvent.md)
 - [Promotion](docs/Promotion.md)
 - [RemovalShipmentAdjustmentEvent](docs/RemovalShipmentAdjustmentEvent.md)
 - [RemovalShipmentEvent](docs/RemovalShipmentEvent.md)
 - [RemovalShipmentItem](docs/RemovalShipmentItem.md)
 - [RemovalShipmentItemAdjustment](docs/RemovalShipmentItemAdjustment.md)
 - [RentalTransactionEvent](docs/RentalTransactionEvent.md)
 - [RetrochargeEvent](docs/RetrochargeEvent.md)
 - [SafetReimbursementEvent](docs/SafetReimbursementEvent.md)
 - [SafetReimbursementItem](docs/SafetReimbursementItem.md)
 - [SellerDealPaymentEvent](docs/SellerDealPaymentEvent.md)
 - [SellerReviewEnrollmentPaymentEvent](docs/SellerReviewEnrollmentPaymentEvent.md)
 - [ServiceFeeEvent](docs/ServiceFeeEvent.md)
 - [ShipmentEvent](docs/ShipmentEvent.md)
 - [ShipmentItem](docs/ShipmentItem.md)
 - [SolutionProviderCreditEvent](docs/SolutionProviderCreditEvent.md)
 - [TaxWithheldComponent](docs/TaxWithheldComponent.md)
 - [TaxWithholdingEvent](docs/TaxWithholdingEvent.md)
 - [TaxWithholdingPeriod](docs/TaxWithholdingPeriod.md)
 - [TdsReimbursementEvent](docs/TdsReimbursementEvent.md)
 - [TrialShipmentEvent](docs/TrialShipmentEvent.md)
 - [ValueAddedServiceChargeEventList](docs/ValueAddedServiceChargeEventList.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



