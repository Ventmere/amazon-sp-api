/*
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderAcknowledgementItem : Details of an individual order being acknowledged.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderAcknowledgementItem {
    /// The purchase order number for this order. Formatting Notes: alpha-numeric code.
    #[serde(rename = "purchaseOrderNumber")]
    pub purchase_order_number: String,
    /// The vendor's order number for this order.
    #[serde(rename = "vendorOrderNumber")]
    pub vendor_order_number: String,
    /// The date and time when the order is acknowledged, in ISO-8601 date/time format. For example: 2018-07-16T23:00:00Z / 2018-07-16T23:00:00-05:00 / 2018-07-16T23:00:00-08:00.
    #[serde(rename = "acknowledgementDate")]
    pub acknowledgement_date: String,
    #[serde(rename = "acknowledgementStatus")]
    pub acknowledgement_status: Box<crate::models::AcknowledgementStatus>,
    #[serde(rename = "sellingParty")]
    pub selling_party: Box<crate::models::PartyIdentification>,
    #[serde(rename = "shipFromParty")]
    pub ship_from_party: Box<crate::models::PartyIdentification>,
    /// Item details including acknowledged quantity.
    #[serde(rename = "itemAcknowledgements")]
    pub item_acknowledgements: Vec<crate::models::OrderItemAcknowledgement>,
}

impl OrderAcknowledgementItem {
    /// Details of an individual order being acknowledged.
    pub fn new(purchase_order_number: String, vendor_order_number: String, acknowledgement_date: String, acknowledgement_status: crate::models::AcknowledgementStatus, selling_party: crate::models::PartyIdentification, ship_from_party: crate::models::PartyIdentification, item_acknowledgements: Vec<crate::models::OrderItemAcknowledgement>) -> OrderAcknowledgementItem {
        OrderAcknowledgementItem {
            purchase_order_number,
            vendor_order_number,
            acknowledgement_date,
            acknowledgement_status: Box::new(acknowledgement_status),
            selling_party: Box::new(selling_party),
            ship_from_party: Box::new(ship_from_party),
            item_acknowledgements,
        }
    }
}


