mod constants;
pub use constants::SpApiEndpoint;
pub mod config;
pub use amazon_sp_api_shared as shared;
pub mod error;
pub mod client;

pub mod apis {
  #[cfg(feature = "aplus_content")]
  pub use amazon_sp_aplus_content as aplus_content;
  #[cfg(feature = "authorization")]
  pub use amazon_sp_authorization as authorization;
  #[cfg(feature = "catalog_items")]
  pub use amazon_sp_catalog_items as catalog_items;
  #[cfg(feature = "easy_ship")]
  pub use amazon_sp_easy_ship as easy_ship;
  #[cfg(feature = "fba_inbound_eligibility")]
  pub use amazon_sp_fba_inbound_eligibility as fba_inbound_eligibility;
  #[cfg(feature = "fba_inventory")]
  pub use amazon_sp_fba_inventory as fba_inventory;
  #[cfg(feature = "fba_small_and_light")]
  pub use amazon_sp_fba_small_and_light as fba_small_and_light;
  #[cfg(feature = "feeds")]
  pub use amazon_sp_feeds as feeds;
  #[cfg(feature = "finances")]
  pub use amazon_sp_finances as finances;
  #[cfg(feature = "fulfillment_inbound")]
  pub use amazon_sp_fulfillment_inbound as fulfillment_inbound;
  #[cfg(feature = "fulfillment_outbound")]
  pub use amazon_sp_fulfillment_outbound as fulfillment_outbound;
  #[cfg(feature = "listings_items")]
  pub use amazon_sp_listings_items as listings_items;
  #[cfg(feature = "listings_restrictions")]
  pub use amazon_sp_listings_restrictions as listings_restrictions;
  #[cfg(feature = "merchant_fulfillment")]
  pub use amazon_sp_merchant_fulfillment as merchant_fulfillment;
  #[cfg(feature = "messaging")]
  pub use amazon_sp_messaging as messaging;
  #[cfg(feature = "notifications")]
  pub use amazon_sp_notifications as notifications;
  #[cfg(feature = "orders")]
  pub use amazon_sp_orders as orders;
  #[cfg(feature = "product_fees")]
  pub use amazon_sp_product_fees as product_fees;
  #[cfg(feature = "product_pricing")]
  pub use amazon_sp_product_pricing as product_pricing;
  #[cfg(feature = "product_type_definitions")]
  pub use amazon_sp_product_type_definitions as product_type_definitions;
  #[cfg(feature = "reports")]
  pub use amazon_sp_reports as reports;
  #[cfg(feature = "sales")]
  pub use amazon_sp_sales as sales;
  #[cfg(feature = "sellers")]
  pub use amazon_sp_sellers as sellers;
  #[cfg(feature = "services")]
  pub use amazon_sp_services as services;
  #[cfg(feature = "shipment_invoicing")]
  pub use amazon_sp_shipment_invoicing as shipment_invoicing;
  #[cfg(feature = "shipping")]
  pub use amazon_sp_shipping as shipping;
  #[cfg(feature = "solicitations")]
  pub use amazon_sp_solicitations as solicitations;
  #[cfg(feature = "tokens")]
  pub use amazon_sp_tokens as tokens;
  #[cfg(feature = "uploads")]
  pub use amazon_sp_uploads as uploads;
  #[cfg(feature = "vendor_direct_fulfillment_inventory")]
  pub use amazon_sp_vendor_direct_fulfillment_inventory as vendor_direct_fulfillment_inventory;
  #[cfg(feature = "vendor_direct_fulfillment_orders")]
  pub use amazon_sp_vendor_direct_fulfillment_orders as vendor_direct_fulfillment_orders;
  #[cfg(feature = "vendor_direct_fulfillment_payments")]
  pub use amazon_sp_vendor_direct_fulfillment_payments as vendor_direct_fulfillment_payments;
  #[cfg(feature = "vendor_direct_fulfillment_sandbox_test_data")]
  pub use amazon_sp_vendor_direct_fulfillment_sandbox_test_data as vendor_direct_fulfillment_sandbox_test_data;
  #[cfg(feature = "vendor_direct_fulfillment_shipping")]
  pub use amazon_sp_vendor_direct_fulfillment_shipping as vendor_direct_fulfillment_shipping;
  #[cfg(feature = "vendor_direct_fulfillment_transactions")]
  pub use amazon_sp_vendor_direct_fulfillment_transactions as vendor_direct_fulfillment_transactions;
  #[cfg(feature = "vendor_invoices")]
  pub use amazon_sp_vendor_invoices as vendor_invoices;
  #[cfg(feature = "vendor_orders")]
  pub use amazon_sp_vendor_orders as vendor_orders;
  #[cfg(feature = "vendor_shipments")]
  pub use amazon_sp_vendor_shipments as vendor_shipments;
  #[cfg(feature = "vendor_transaction_status")]
  pub use amazon_sp_vendor_transaction_status as vendor_transaction_status;
}