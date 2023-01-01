/*
 * Selling Partner API for Direct Fulfillment Inventory Updates
 *
 * The Selling Partner API for Direct Fulfillment Inventory Updates provides programmatic access to a direct fulfillment vendor's inventory updates.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionReference {
    /// GUID to identify this transaction. This value can be used with the Transaction Status API to return the status of this transaction.
    #[serde(default, rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl TransactionReference {
    pub fn new() -> TransactionReference {
        TransactionReference {
            transaction_id: None,
        }
    }
}


