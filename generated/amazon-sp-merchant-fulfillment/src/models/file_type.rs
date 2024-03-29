/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FileType : The file type for a label.

/// The file type for a label.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileType {
    #[serde(rename = "application/pdf")]
    ApplicationPdf,
    #[serde(rename = "application/zpl")]
    ApplicationZpl,
    #[serde(rename = "image/png")]
    ImagePng,

}

impl ToString for FileType {
    fn to_string(&self) -> String {
        match self {
            Self::ApplicationPdf => String::from("application/pdf"),
            Self::ApplicationZpl => String::from("application/zpl"),
            Self::ImagePng => String::from("image/png"),
        }
    }
}

impl Default for FileType {
    fn default() -> FileType {
        Self::ApplicationPdf
    }
}




