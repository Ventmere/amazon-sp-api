/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentExecutionDetailItem : Information about a sub-payment method used to pay for a COD order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentExecutionDetailItem {
    #[serde(rename = "Payment")]
    pub payment: Box<crate::models::Money>,
    /// A sub-payment method for a COD order.  Possible values:  * COD - Cash On Delivery.  * GC - Gift Card.  * PointsAccount - Amazon Points.
    #[serde(rename = "PaymentMethod")]
    pub payment_method: String,
}

impl PaymentExecutionDetailItem {
    /// Information about a sub-payment method used to pay for a COD order.
    pub fn new(payment: crate::models::Money, payment_method: String) -> PaymentExecutionDetailItem {
        PaymentExecutionDetailItem {
            payment: Box::new(payment),
            payment_method,
        }
    }
}


