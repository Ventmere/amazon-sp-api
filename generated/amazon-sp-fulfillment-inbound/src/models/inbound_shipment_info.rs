/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InboundShipmentInfo : Information about the seller's inbound shipments. Returned by the listInboundShipments operation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InboundShipmentInfo {
    /// The shipment identifier submitted in the request.
    #[serde(rename = "ShipmentId", skip_serializing_if = "Option::is_none")]
    pub shipment_id: Option<String>,
    /// The name for the inbound shipment.
    #[serde(rename = "ShipmentName", skip_serializing_if = "Option::is_none")]
    pub shipment_name: Option<String>,
    #[serde(rename = "ShipFromAddress")]
    pub ship_from_address: Box<crate::models::Address>,
    /// An Amazon fulfillment center identifier created by Amazon.
    #[serde(rename = "DestinationFulfillmentCenterId", skip_serializing_if = "Option::is_none")]
    pub destination_fulfillment_center_id: Option<String>,
    #[serde(rename = "ShipmentStatus", skip_serializing_if = "Option::is_none")]
    pub shipment_status: Option<crate::models::ShipmentStatus>,
    #[serde(rename = "LabelPrepType", skip_serializing_if = "Option::is_none")]
    pub label_prep_type: Option<crate::models::LabelPrepType>,
    /// Indicates whether or not an inbound shipment contains case-packed boxes. When AreCasesRequired = true for an inbound shipment, all items in the inbound shipment must be case packed.
    #[serde(rename = "AreCasesRequired")]
    pub are_cases_required: bool,
    #[serde(rename = "ConfirmedNeedByDate", skip_serializing_if = "Option::is_none")]
    pub confirmed_need_by_date: Option<String>,
    #[serde(rename = "BoxContentsSource", skip_serializing_if = "Option::is_none")]
    pub box_contents_source: Option<crate::models::BoxContentsSource>,
    #[serde(rename = "EstimatedBoxContentsFee", skip_serializing_if = "Option::is_none")]
    pub estimated_box_contents_fee: Option<Box<crate::models::BoxContentsFeeDetails>>,
}

impl InboundShipmentInfo {
    /// Information about the seller's inbound shipments. Returned by the listInboundShipments operation.
    pub fn new(ship_from_address: crate::models::Address, are_cases_required: bool) -> InboundShipmentInfo {
        InboundShipmentInfo {
            shipment_id: None,
            shipment_name: None,
            ship_from_address: Box::new(ship_from_address),
            destination_fulfillment_center_id: None,
            shipment_status: None,
            label_prep_type: None,
            are_cases_required,
            confirmed_need_by_date: None,
            box_contents_source: None,
            estimated_box_contents_fee: None,
        }
    }
}

