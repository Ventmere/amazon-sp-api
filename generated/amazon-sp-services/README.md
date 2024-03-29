# Rust API client for amazon-sp-services

With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.

For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `amazon-sp-services` and add the following to `Cargo.toml` under `[dependencies]`:

```
amazon-sp-services = { path = "./amazon-sp-services" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ServiceApi* | [**add_appointment_for_service_job_by_service_job_id**](docs/ServiceApi.md#add_appointment_for_service_job_by_service_job_id) | **POST** /service/v1/serviceJobs/{serviceJobId}/appointments | 
*ServiceApi* | [**assign_appointment_resources**](docs/ServiceApi.md#assign_appointment_resources) | **PUT** /service/v1/serviceJobs/{serviceJobId}/appointments/{appointmentId}/resources | 
*ServiceApi* | [**cancel_reservation**](docs/ServiceApi.md#cancel_reservation) | **DELETE** /service/v1/reservation/{reservationId} | 
*ServiceApi* | [**cancel_service_job_by_service_job_id**](docs/ServiceApi.md#cancel_service_job_by_service_job_id) | **PUT** /service/v1/serviceJobs/{serviceJobId}/cancellations | 
*ServiceApi* | [**complete_service_job_by_service_job_id**](docs/ServiceApi.md#complete_service_job_by_service_job_id) | **PUT** /service/v1/serviceJobs/{serviceJobId}/completions | 
*ServiceApi* | [**create_reservation**](docs/ServiceApi.md#create_reservation) | **POST** /service/v1/reservation | 
*ServiceApi* | [**create_service_document_upload_destination**](docs/ServiceApi.md#create_service_document_upload_destination) | **POST** /service/v1/documents | 
*ServiceApi* | [**get_appointment_slots**](docs/ServiceApi.md#get_appointment_slots) | **GET** /service/v1/appointmentSlots | 
*ServiceApi* | [**get_appointmment_slots_by_job_id**](docs/ServiceApi.md#get_appointmment_slots_by_job_id) | **GET** /service/v1/serviceJobs/{serviceJobId}/appointmentSlots | 
*ServiceApi* | [**get_fixed_slot_capacity**](docs/ServiceApi.md#get_fixed_slot_capacity) | **POST** /service/v1/serviceResources/{resourceId}/capacity/fixed | 
*ServiceApi* | [**get_range_slot_capacity**](docs/ServiceApi.md#get_range_slot_capacity) | **POST** /service/v1/serviceResources/{resourceId}/capacity/range | 
*ServiceApi* | [**get_service_job_by_service_job_id**](docs/ServiceApi.md#get_service_job_by_service_job_id) | **GET** /service/v1/serviceJobs/{serviceJobId} | 
*ServiceApi* | [**get_service_jobs**](docs/ServiceApi.md#get_service_jobs) | **GET** /service/v1/serviceJobs | 
*ServiceApi* | [**reschedule_appointment_for_service_job_by_service_job_id**](docs/ServiceApi.md#reschedule_appointment_for_service_job_by_service_job_id) | **POST** /service/v1/serviceJobs/{serviceJobId}/appointments/{appointmentId} | 
*ServiceApi* | [**set_appointment_fulfillment_data**](docs/ServiceApi.md#set_appointment_fulfillment_data) | **PUT** /service/v1/serviceJobs/{serviceJobId}/appointments/{appointmentId}/fulfillment | 
*ServiceApi* | [**update_reservation**](docs/ServiceApi.md#update_reservation) | **PUT** /service/v1/reservation/{reservationId} | 
*ServiceApi* | [**update_schedule**](docs/ServiceApi.md#update_schedule) | **PUT** /service/v1/serviceResources/{resourceId}/schedules | 


## Documentation For Models

 - [AddAppointmentRequest](docs/AddAppointmentRequest.md)
 - [Address](docs/Address.md)
 - [Appointment](docs/Appointment.md)
 - [AppointmentResource](docs/AppointmentResource.md)
 - [AppointmentSlot](docs/AppointmentSlot.md)
 - [AppointmentSlotReport](docs/AppointmentSlotReport.md)
 - [AppointmentTime](docs/AppointmentTime.md)
 - [AppointmentTimeInput](docs/AppointmentTimeInput.md)
 - [AssignAppointmentResourcesRequest](docs/AssignAppointmentResourcesRequest.md)
 - [AssignAppointmentResourcesResponse](docs/AssignAppointmentResourcesResponse.md)
 - [AssignAppointmentResourcesResponsePayload](docs/AssignAppointmentResourcesResponsePayload.md)
 - [AssociatedItem](docs/AssociatedItem.md)
 - [AvailabilityRecord](docs/AvailabilityRecord.md)
 - [Buyer](docs/Buyer.md)
 - [CancelReservationResponse](docs/CancelReservationResponse.md)
 - [CancelServiceJobByServiceJobIdResponse](docs/CancelServiceJobByServiceJobIdResponse.md)
 - [CapacityType](docs/CapacityType.md)
 - [CompleteServiceJobByServiceJobIdResponse](docs/CompleteServiceJobByServiceJobIdResponse.md)
 - [CreateReservationRecord](docs/CreateReservationRecord.md)
 - [CreateReservationRequest](docs/CreateReservationRequest.md)
 - [CreateReservationResponse](docs/CreateReservationResponse.md)
 - [CreateServiceDocumentUploadDestination](docs/CreateServiceDocumentUploadDestination.md)
 - [DayOfWeek](docs/DayOfWeek.md)
 - [EncryptionDetails](docs/EncryptionDetails.md)
 - [Error](docs/Error.md)
 - [FixedSlot](docs/FixedSlot.md)
 - [FixedSlotCapacity](docs/FixedSlotCapacity.md)
 - [FixedSlotCapacityErrors](docs/FixedSlotCapacityErrors.md)
 - [FixedSlotCapacityQuery](docs/FixedSlotCapacityQuery.md)
 - [FulfillmentDocument](docs/FulfillmentDocument.md)
 - [FulfillmentTime](docs/FulfillmentTime.md)
 - [GetAppointmentSlotsResponse](docs/GetAppointmentSlotsResponse.md)
 - [GetServiceJobByServiceJobIdResponse](docs/GetServiceJobByServiceJobIdResponse.md)
 - [GetServiceJobsResponse](docs/GetServiceJobsResponse.md)
 - [ItemDelivery](docs/ItemDelivery.md)
 - [ItemDeliveryPromise](docs/ItemDeliveryPromise.md)
 - [JobListing](docs/JobListing.md)
 - [Poa](docs/Poa.md)
 - [RangeCapacity](docs/RangeCapacity.md)
 - [RangeSlot](docs/RangeSlot.md)
 - [RangeSlotCapacity](docs/RangeSlotCapacity.md)
 - [RangeSlotCapacityErrors](docs/RangeSlotCapacityErrors.md)
 - [RangeSlotCapacityQuery](docs/RangeSlotCapacityQuery.md)
 - [Recurrence](docs/Recurrence.md)
 - [RescheduleAppointmentRequest](docs/RescheduleAppointmentRequest.md)
 - [Reservation](docs/Reservation.md)
 - [ScopeOfWork](docs/ScopeOfWork.md)
 - [Seller](docs/Seller.md)
 - [ServiceDocumentUploadDestination](docs/ServiceDocumentUploadDestination.md)
 - [ServiceJob](docs/ServiceJob.md)
 - [ServiceJobProvider](docs/ServiceJobProvider.md)
 - [ServiceLocation](docs/ServiceLocation.md)
 - [ServiceUploadDocument](docs/ServiceUploadDocument.md)
 - [SetAppointmentFulfillmentDataRequest](docs/SetAppointmentFulfillmentDataRequest.md)
 - [SetAppointmentResponse](docs/SetAppointmentResponse.md)
 - [Technician](docs/Technician.md)
 - [UpdateReservationRecord](docs/UpdateReservationRecord.md)
 - [UpdateReservationRequest](docs/UpdateReservationRequest.md)
 - [UpdateReservationResponse](docs/UpdateReservationResponse.md)
 - [UpdateScheduleRecord](docs/UpdateScheduleRecord.md)
 - [UpdateScheduleRequest](docs/UpdateScheduleRequest.md)
 - [UpdateScheduleResponse](docs/UpdateScheduleResponse.md)
 - [Warning](docs/Warning.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



