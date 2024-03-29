/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomerInvoice {
    /// The purchase order number for this order.
    #[serde(default, rename = "purchaseOrderNumber")]
    pub purchase_order_number: String,
    /// The Base64encoded customer invoice.
    #[serde(default, rename = "content")]
    pub content: String,
}

impl CustomerInvoice {
    pub fn new(purchase_order_number: String, content: String) -> CustomerInvoice {
        CustomerInvoice {
            purchase_order_number,
            content,
        }
    }
}


