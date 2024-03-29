/*
 * Selling Partner API for Easy Ship
 *
 * The Selling Partner API for Easy Ship helps you build applications that help sellers manage and ship Amazon Easy Ship orders.  Your Easy Ship applications can:  * Get available time slots for packages to be scheduled for delivery.  * Schedule, reschedule, and cancel Easy Ship orders.  * Print labels, invoices, and warranties.  See the [Marketplace Support Table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table) for the differences in Easy Ship operations by marketplace.
 *
 * The version of the OpenAPI document: 2022-03-23
 * Contact: marketplaceapitest@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// InvoiceData : Invoice number and date.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceData {
    /// A string of up to 255 characters.
    #[serde(default, rename = "invoiceNumber")]
    pub invoice_number: String,
    /// A datetime value in ISO 8601 format.
    #[serde(default, rename = "invoiceDate", skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<String>,
}

impl InvoiceData {
    /// Invoice number and date.
    pub fn new(invoice_number: String) -> InvoiceData {
        InvoiceData {
            invoice_number,
            invoice_date: None,
        }
    }
}


