/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateFulfillmentOrderRequest {
    /// The marketplace the fulfillment order is placed against.
    #[serde(default, rename = "marketplaceId", skip_serializing_if = "Option::is_none")]
    pub marketplace_id: Option<String>,
    /// A fulfillment order identifier that the seller creates. This value displays as the order identifier in recipient-facing materials such as the outbound shipment packing slip. The value of DisplayableOrderId should match the order identifier that the seller provides to the recipient. The seller can use the SellerFulfillmentOrderId for this value or they can specify an alternate value if they want the recipient to reference an alternate order identifier.
    #[serde(default, rename = "displayableOrderId", skip_serializing_if = "Option::is_none")]
    pub displayable_order_id: Option<String>,
    #[serde(default, rename = "displayableOrderDate", skip_serializing_if = "Option::is_none")]
    pub displayable_order_date: Option<String>,
    /// Order-specific text that appears in recipient-facing materials such as the outbound shipment packing slip.
    #[serde(default, rename = "displayableOrderComment", skip_serializing_if = "Option::is_none")]
    pub displayable_order_comment: Option<String>,
    #[serde(default, rename = "shippingSpeedCategory", skip_serializing_if = "Option::is_none")]
    pub shipping_speed_category: Option<crate::models::ShippingSpeedCategory>,
    #[serde(default, rename = "destinationAddress", skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<Box<crate::models::Address>>,
    #[serde(default, rename = "fulfillmentAction", skip_serializing_if = "Option::is_none")]
    pub fulfillment_action: Option<crate::models::FulfillmentAction>,
    #[serde(default, rename = "fulfillmentPolicy", skip_serializing_if = "Option::is_none")]
    pub fulfillment_policy: Option<crate::models::FulfillmentPolicy>,
    /// The two-character country code for the country from which the fulfillment order ships. Must be in ISO 3166-1 alpha-2 format.
    #[serde(default, rename = "shipFromCountryCode", skip_serializing_if = "Option::is_none")]
    pub ship_from_country_code: Option<String>,
    /// A list of email addresses that the seller provides that are used by Amazon to send ship-complete notifications to recipients on behalf of the seller.
    #[serde(default, rename = "notificationEmails", skip_serializing_if = "Option::is_none")]
    pub notification_emails: Option<Vec<String>>,
    /// A list of features and their fulfillment policies to apply to the order.
    #[serde(default, rename = "featureConstraints", skip_serializing_if = "Option::is_none")]
    pub feature_constraints: Option<Vec<crate::models::FeatureSettings>>,
    /// An array of fulfillment order item information for updating a fulfillment order.
    #[serde(default, rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::UpdateFulfillmentOrderItem>>,
}

impl UpdateFulfillmentOrderRequest {
    pub fn new() -> UpdateFulfillmentOrderRequest {
        UpdateFulfillmentOrderRequest {
            marketplace_id: None,
            displayable_order_id: None,
            displayable_order_date: None,
            displayable_order_comment: None,
            shipping_speed_category: None,
            destination_address: None,
            fulfillment_action: None,
            fulfillment_policy: None,
            ship_from_country_code: None,
            notification_emails: None,
            feature_constraints: None,
            items: None,
        }
    }
}


