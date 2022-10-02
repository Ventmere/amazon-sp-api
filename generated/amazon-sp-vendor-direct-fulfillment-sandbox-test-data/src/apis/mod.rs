pub use amazon_sp_api_shared::error::SharedError as Error;
pub use amazon_sp_api_shared::request::ResponseContent;

pub mod vendor_df_sandbox_api;
pub mod vendor_df_sandboxtransactionstatus_api;

pub mod configuration;

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}