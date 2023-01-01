/*
 * Selling Partner API for Shipment Invoicing
 *
 * The Selling Partner API for Shipment Invoicing helps you programmatically retrieve shipment invoice information in the Brazil marketplace for a selling partner’s Fulfillment by Amazon (FBA) orders.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SubmitInvoiceRequest : The request schema for the submitInvoice operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitInvoiceRequest {
    /// Shipment invoice document content.
    #[serde(default, rename = "InvoiceContent")]
    pub invoice_content: String,
    /// An Amazon marketplace identifier.
    #[serde(default, rename = "MarketplaceId", skip_serializing_if = "Option::is_none")]
    pub marketplace_id: Option<String>,
    /// MD5 sum for validating the invoice data. For more information about calculating this value, see [Working with Content-MD5 Checksums](https://docs.developer.amazonservices.com/en_US/dev_guide/DG_MD5.html).
    #[serde(default, rename = "ContentMD5Value")]
    pub content_md5_value: String,
}

impl SubmitInvoiceRequest {
    /// The request schema for the submitInvoice operation.
    pub fn new(invoice_content: String, content_md5_value: String) -> SubmitInvoiceRequest {
        SubmitInvoiceRequest {
            invoice_content,
            marketplace_id: None,
            content_md5_value,
        }
    }
}


