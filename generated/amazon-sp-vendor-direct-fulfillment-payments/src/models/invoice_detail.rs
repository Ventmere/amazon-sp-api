/*
 * Selling Partner API for Direct Fulfillment Payments
 *
 * The Selling Partner API for Direct Fulfillment Payments provides programmatic access to a direct fulfillment vendor's invoice data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceDetail {
    /// The unique invoice number.
    #[serde(default, rename = "invoiceNumber")]
    pub invoice_number: String,
    /// Invoice date.
    #[serde(default, rename = "invoiceDate")]
    pub invoice_date: String,
    /// An additional unique reference number used for regulatory or other purposes.
    #[serde(default, rename = "referenceNumber", skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
    #[serde(default, rename = "remitToParty")]
    pub remit_to_party: Box<crate::models::PartyIdentification>,
    #[serde(default, rename = "shipFromParty")]
    pub ship_from_party: Box<crate::models::PartyIdentification>,
    #[serde(default, rename = "billToParty", skip_serializing_if = "Option::is_none")]
    pub bill_to_party: Option<Box<crate::models::PartyIdentification>>,
    /// Ship-to country code.
    #[serde(default, rename = "shipToCountryCode", skip_serializing_if = "Option::is_none")]
    pub ship_to_country_code: Option<String>,
    /// The payment terms for the invoice.
    #[serde(default, rename = "paymentTermsCode", skip_serializing_if = "Option::is_none")]
    pub payment_terms_code: Option<String>,
    #[serde(default, rename = "invoiceTotal")]
    pub invoice_total: Box<crate::models::Money>,
    /// Individual tax details per line item.
    #[serde(default, rename = "taxTotals", skip_serializing_if = "Option::is_none")]
    pub tax_totals: Option<Vec<crate::models::TaxDetail>>,
    /// Additional details provided by the selling party, for tax related or other purposes.
    #[serde(default, rename = "additionalDetails", skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<Vec<crate::models::AdditionalDetails>>,
    /// Total charge amount details for all line items.
    #[serde(default, rename = "chargeDetails", skip_serializing_if = "Option::is_none")]
    pub charge_details: Option<Vec<crate::models::ChargeDetails>>,
    /// Provides the details of the items in this invoice.
    #[serde(default, rename = "items")]
    pub items: Vec<crate::models::InvoiceItem>,
}

impl InvoiceDetail {
    pub fn new(invoice_number: String, invoice_date: String, remit_to_party: crate::models::PartyIdentification, ship_from_party: crate::models::PartyIdentification, invoice_total: crate::models::Money, items: Vec<crate::models::InvoiceItem>) -> InvoiceDetail {
        InvoiceDetail {
            invoice_number,
            invoice_date,
            reference_number: None,
            remit_to_party: Box::new(remit_to_party),
            ship_from_party: Box::new(ship_from_party),
            bill_to_party: None,
            ship_to_country_code: None,
            payment_terms_code: None,
            invoice_total: Box::new(invoice_total),
            tax_totals: None,
            additional_details: None,
            charge_details: None,
            items,
        }
    }
}


