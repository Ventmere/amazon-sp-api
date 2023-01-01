/*
 * Selling Partner API for Shipment Invoicing
 *
 * The Selling Partner API for Shipment Invoicing helps you programmatically retrieve shipment invoice information in the Brazil marketplace for a selling partner’s Fulfillment by Amazon (FBA) orders.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Address : The shipping address details of the shipment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Address {
    /// The name.
    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The street address.
    #[serde(default, rename = "AddressLine1", skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// Additional street address information, if required.
    #[serde(default, rename = "AddressLine2", skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// Additional street address information, if required.
    #[serde(default, rename = "AddressLine3", skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    /// The city.
    #[serde(default, rename = "City", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The county.
    #[serde(default, rename = "County", skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    /// The district.
    #[serde(default, rename = "District", skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// The state or region.
    #[serde(default, rename = "StateOrRegion", skip_serializing_if = "Option::is_none")]
    pub state_or_region: Option<String>,
    /// The postal code.
    #[serde(default, rename = "PostalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The country code.
    #[serde(default, rename = "CountryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The phone number.
    #[serde(default, rename = "Phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, rename = "AddressType", skip_serializing_if = "Option::is_none")]
    pub address_type: Option<crate::models::AddressTypeEnum>,
}

impl Address {
    /// The shipping address details of the shipment.
    pub fn new() -> Address {
        Address {
            name: None,
            address_line1: None,
            address_line2: None,
            address_line3: None,
            city: None,
            county: None,
            district: None,
            state_or_region: None,
            postal_code: None,
            country_code: None,
            phone: None,
            address_type: None,
        }
    }
}


