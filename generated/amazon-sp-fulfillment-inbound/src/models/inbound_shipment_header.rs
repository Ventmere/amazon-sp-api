/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InboundShipmentHeader : Inbound shipment information used to create and update inbound shipments.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InboundShipmentHeader {
    /// The name for the shipment. Use a naming convention that helps distinguish between shipments over time, such as the date the shipment was created.
    #[serde(rename = "ShipmentName")]
    pub shipment_name: String,
    #[serde(rename = "ShipFromAddress")]
    pub ship_from_address: Box<crate::models::Address>,
    /// The identifier for the fulfillment center to which the shipment will be shipped. Get this value from the InboundShipmentPlan object in the response returned by the createInboundShipmentPlan operation.
    #[serde(rename = "DestinationFulfillmentCenterId")]
    pub destination_fulfillment_center_id: String,
    /// Indicates whether or not an inbound shipment contains case-packed boxes. Note: A shipment must contain either all case-packed boxes or all individually packed boxes.  Possible values:  true - All boxes in the shipment must be case packed.  false - All boxes in the shipment must be individually packed.  Note: If AreCasesRequired = true for an inbound shipment, then the value of QuantityInCase must be greater than zero for every item in the shipment. Otherwise the service returns an error.
    #[serde(rename = "AreCasesRequired", skip_serializing_if = "Option::is_none")]
    pub are_cases_required: Option<bool>,
    #[serde(rename = "ShipmentStatus")]
    pub shipment_status: crate::models::ShipmentStatus,
    #[serde(rename = "LabelPrepPreference")]
    pub label_prep_preference: crate::models::LabelPrepPreference,
    #[serde(rename = "IntendedBoxContentsSource", skip_serializing_if = "Option::is_none")]
    pub intended_box_contents_source: Option<crate::models::IntendedBoxContentsSource>,
}

impl InboundShipmentHeader {
    /// Inbound shipment information used to create and update inbound shipments.
    pub fn new(shipment_name: String, ship_from_address: crate::models::Address, destination_fulfillment_center_id: String, shipment_status: crate::models::ShipmentStatus, label_prep_preference: crate::models::LabelPrepPreference) -> InboundShipmentHeader {
        InboundShipmentHeader {
            shipment_name,
            ship_from_address: Box::new(ship_from_address),
            destination_fulfillment_center_id,
            are_cases_required: None,
            shipment_status,
            label_prep_preference,
            intended_box_contents_source: None,
        }
    }
}

