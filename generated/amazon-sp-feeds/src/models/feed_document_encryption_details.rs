/*
 * Selling Partner API for Feeds
 *
 * The Selling Partner API for Feeds lets you upload data to Amazon on behalf of a selling partner.
 *
 * The version of the OpenAPI document: 2020-09-04
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FeedDocumentEncryptionDetails : Encryption details for required client-side encryption and decryption of document contents.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedDocumentEncryptionDetails {
    /// The encryption standard required to encrypt or decrypt the document contents.
    #[serde(rename = "standard")]
    pub standard: Standard,
    /// The vector to encrypt or decrypt the document contents using Cipher Block Chaining (CBC).
    #[serde(rename = "initializationVector")]
    pub initialization_vector: String,
    /// The encryption key used to encrypt or decrypt the document contents.
    #[serde(rename = "key")]
    pub key: String,
}

impl FeedDocumentEncryptionDetails {
    /// Encryption details for required client-side encryption and decryption of document contents.
    pub fn new(standard: Standard, initialization_vector: String, key: String) -> FeedDocumentEncryptionDetails {
        FeedDocumentEncryptionDetails {
            standard,
            initialization_vector,
            key,
        }
    }
}

/// The encryption standard required to encrypt or decrypt the document contents.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Standard {
    #[serde(rename = "AES")]
    AES,
}

impl Default for Standard {
    fn default() -> Standard {
        Self::AES
    }
}

