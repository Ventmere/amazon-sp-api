/*
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders and manage their resources.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServiceUploadDocument : Input for to be uploaded document.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceUploadDocument {
    /// The content type of the to-be-uploaded file
    #[serde(default, rename = "contentType")]
    pub content_type: ContentType,
    /// The content length of the to-be-uploaded file
    #[serde(default, rename = "contentLength")]
    pub content_length: f32,
    /// An MD5 hash of the content to be submitted to the upload destination. This value is used to determine if the data has been corrupted or tampered with during transit.
    #[serde(default, rename = "contentMD5", skip_serializing_if = "Option::is_none")]
    pub content_md5: Option<String>,
}

impl ServiceUploadDocument {
    /// Input for to be uploaded document.
    pub fn new(content_type: ContentType, content_length: f32) -> ServiceUploadDocument {
        ServiceUploadDocument {
            content_type,
            content_length,
            content_md5: None,
        }
    }
}

/// The content type of the to-be-uploaded file
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "TIFF")]
    TIFF,
    #[serde(rename = "JPG")]
    JPG,
    #[serde(rename = "PNG")]
    PNG,
    #[serde(rename = "JPEG")]
    JPEG,
    #[serde(rename = "GIF")]
    GIF,
    #[serde(rename = "PDF")]
    PDF,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::TIFF
    }
}

