/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Address : The postal address information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Address {
    /// The name of the addressee, or business name.
    #[serde(default, rename = "Name")]
    pub name: String,
    /// The street address information.
    #[serde(default, rename = "AddressLine1")]
    pub address_line1: String,
    /// Additional street address information.
    #[serde(default, rename = "AddressLine2", skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// Additional street address information.
    #[serde(default, rename = "AddressLine3", skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    /// The district or county.
    #[serde(default, rename = "DistrictOrCounty", skip_serializing_if = "Option::is_none")]
    pub district_or_county: Option<String>,
    /// The email address.
    #[serde(default, rename = "Email")]
    pub email: String,
    /// The city.
    #[serde(default, rename = "City")]
    pub city: String,
    /// The state or province code. **Note.** Required in the Canada, US, and UK marketplaces. Also required for shipments originating from China.
    #[serde(default, rename = "StateOrProvinceCode", skip_serializing_if = "Option::is_none")]
    pub state_or_province_code: Option<String>,
    /// The zip code or postal code.
    #[serde(default, rename = "PostalCode")]
    pub postal_code: String,
    /// The country code. A two-character country code, in ISO 3166-1 alpha-2 format.
    #[serde(default, rename = "CountryCode")]
    pub country_code: String,
    /// The phone number.
    #[serde(default, rename = "Phone")]
    pub phone: String,
}

impl Address {
    /// The postal address information.
    pub fn new(name: String, address_line1: String, email: String, city: String, postal_code: String, country_code: String, phone: String) -> Address {
        Address {
            name,
            address_line1,
            address_line2: None,
            address_line3: None,
            district_or_county: None,
            email,
            city,
            state_or_province_code: None,
            postal_code,
            country_code,
            phone,
        }
    }
}


