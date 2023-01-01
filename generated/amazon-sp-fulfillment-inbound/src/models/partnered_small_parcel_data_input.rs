/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartneredSmallParcelDataInput : Information that is required by an Amazon-partnered carrier to ship a Small Parcel inbound shipment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartneredSmallParcelDataInput {
    /// A list of dimensions and weight information for packages.
    #[serde(default, rename = "PackageList", skip_serializing_if = "Option::is_none")]
    pub package_list: Option<Vec<crate::models::PartneredSmallParcelPackageInput>>,
    /// The Amazon-partnered carrier to use for the inbound shipment. **`CarrierName`** values in France (FR), Italy (IT), Spain (ES), the United Kingdom (UK), and the United States (US): `UNITED_PARCEL_SERVICE_INC`. <br> **`CarrierName`** values in Germany (DE): `DHL_STANDARD`,`UNITED_PARCEL_SERVICE_INC`. <br>Default: `UNITED_PARCEL_SERVICE_INC`.
    #[serde(default, rename = "CarrierName", skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
}

impl PartneredSmallParcelDataInput {
    /// Information that is required by an Amazon-partnered carrier to ship a Small Parcel inbound shipment.
    pub fn new() -> PartneredSmallParcelDataInput {
        PartneredSmallParcelDataInput {
            package_list: None,
            carrier_name: None,
        }
    }
}


