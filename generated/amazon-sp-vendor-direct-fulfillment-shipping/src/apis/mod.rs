pub use amazon_sp_api_shared::error::SharedError as Error;

pub mod customer_invoices_api;
pub mod vendor_shipping_api;
pub mod vendor_shipping_labels_api;

pub mod configuration;

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}