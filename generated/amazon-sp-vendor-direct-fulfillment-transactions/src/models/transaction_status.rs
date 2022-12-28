/*
 * Selling Partner API for Direct Fulfillment Transaction Status
 *
 * The Selling Partner API for Direct Fulfillment Transaction Status provides programmatic access to a direct fulfillment vendor's transaction status.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionStatus : The payload for the getTransactionStatus operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionStatus {
    #[serde(rename = "transactionStatus", skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<Box<crate::models::Transaction>>,
}

impl TransactionStatus {
    /// The payload for the getTransactionStatus operation.
    pub fn new() -> TransactionStatus {
        TransactionStatus {
            transaction_status: None,
        }
    }
}

