/*
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * The version of the OpenAPI document: 2021-12-28
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LabelData : Details of the shipment label.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LabelData {
    /// Identifier for the package. The first package will be 001, the second 002, and so on. This number is used as a reference to refer to this package from the pallet level.
    #[serde(rename = "packageIdentifier", skip_serializing_if = "Option::is_none")]
    pub package_identifier: Option<String>,
    /// Package tracking identifier from the shipping carrier.
    #[serde(rename = "trackingNumber", skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    /// Ship method to be used for shipping the order. Amazon defines Ship Method Codes indicating shipping carrier and shipment service level. Ship Method Codes are case and format sensitive. The same ship method code should returned on the shipment confirmation. Note that the Ship Method Codes are vendor specific and will be provided to each vendor during the implementation.
    #[serde(rename = "shipMethod", skip_serializing_if = "Option::is_none")]
    pub ship_method: Option<String>,
    /// Shipping method name for internal reference.
    #[serde(rename = "shipMethodName", skip_serializing_if = "Option::is_none")]
    pub ship_method_name: Option<String>,
    /// This field will contain the Base64encoded string of the shipment label content.
    #[serde(rename = "content")]
    pub content: String,
}

impl LabelData {
    /// Details of the shipment label.
    pub fn new(content: String) -> LabelData {
        LabelData {
            package_identifier: None,
            tracking_number: None,
            ship_method: None,
            ship_method_name: None,
            content,
        }
    }
}


