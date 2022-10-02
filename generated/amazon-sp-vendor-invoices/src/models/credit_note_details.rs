/*
 * Selling Partner API for Retail Procurement Payments
 *
 * The Selling Partner API for Retail Procurement Payments provides programmatic access to vendors payments data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditNoteDetails : References required in order to process a credit note. This information is required only if InvoiceType is CreditNote.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditNoteDetails {
    /// Original Invoice Number when sending a credit note relating to an existing invoice. One Invoice only to be processed per Credit Note. This is mandatory for AP Credit Notes.
    #[serde(rename = "referenceInvoiceNumber", skip_serializing_if = "Option::is_none")]
    pub reference_invoice_number: Option<String>,
    /// Debit Note Number as generated by Amazon. Recommended for Returns and COOP Credit Notes.
    #[serde(rename = "debitNoteNumber", skip_serializing_if = "Option::is_none")]
    pub debit_note_number: Option<String>,
    /// Identifies the Returns Notice Number. Mandatory for all Returns Credit Notes.
    #[serde(rename = "returnsReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub returns_reference_number: Option<String>,
    /// Defines a date and time according to ISO8601.
    #[serde(rename = "goodsReturnDate", skip_serializing_if = "Option::is_none")]
    pub goods_return_date: Option<String>,
    /// Identifies the Returned Merchandise Authorization ID, if generated.
    #[serde(rename = "rmaId", skip_serializing_if = "Option::is_none")]
    pub rma_id: Option<String>,
    /// Identifies the COOP reference used for COOP agreement. Failure to provide the COOP reference number or the Debit Note number may lead to a rejection of the Credit Note.
    #[serde(rename = "coopReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub coop_reference_number: Option<String>,
    /// Identifies the consignor reference number (VRET number), if generated by Amazon.
    #[serde(rename = "consignorsReferenceNumber", skip_serializing_if = "Option::is_none")]
    pub consignors_reference_number: Option<String>,
}

impl CreditNoteDetails {
    /// References required in order to process a credit note. This information is required only if InvoiceType is CreditNote.
    pub fn new() -> CreditNoteDetails {
        CreditNoteDetails {
            reference_invoice_number: None,
            debit_note_number: None,
            returns_reference_number: None,
            goods_return_date: None,
            rma_id: None,
            coop_reference_number: None,
            consignors_reference_number: None,
        }
    }
}


