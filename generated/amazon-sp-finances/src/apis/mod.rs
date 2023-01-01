pub use amazon_sp_api_shared::error::SharedError as Error;

pub mod default_api;

pub mod configuration;

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}