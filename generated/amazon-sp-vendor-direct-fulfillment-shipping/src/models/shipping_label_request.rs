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
pub struct ShippingLabelRequest {
    /// Purchase order number of the order for which to create a shipping label.
    #[serde(default, rename = "purchaseOrderNumber")]
    pub purchase_order_number: String,
    #[serde(default, rename = "sellingParty")]
    pub selling_party: Box<crate::models::PartyIdentification>,
    #[serde(default, rename = "shipFromParty")]
    pub ship_from_party: Box<crate::models::PartyIdentification>,
    /// A list of the packages in this shipment.
    #[serde(default, rename = "containers", skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<crate::models::Container>>,
}

impl ShippingLabelRequest {
    pub fn new(purchase_order_number: String, selling_party: crate::models::PartyIdentification, ship_from_party: crate::models::PartyIdentification) -> ShippingLabelRequest {
        ShippingLabelRequest {
            purchase_order_number,
            selling_party: Box::new(selling_party),
            ship_from_party: Box::new(ship_from_party),
            containers: None,
        }
    }
}


