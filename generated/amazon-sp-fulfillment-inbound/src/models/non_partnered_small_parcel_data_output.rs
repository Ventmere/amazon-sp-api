/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NonPartneredSmallParcelDataOutput : Information returned by Amazon about a Small Parcel shipment by a carrier that has not partnered with Amazon.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NonPartneredSmallParcelDataOutput {
    /// A list of packages, including carrier, tracking number, and status information for each package.
    #[serde(default, rename = "PackageList")]
    pub package_list: Vec<crate::models::NonPartneredSmallParcelPackageOutput>,
}

impl NonPartneredSmallParcelDataOutput {
    /// Information returned by Amazon about a Small Parcel shipment by a carrier that has not partnered with Amazon.
    pub fn new(package_list: Vec<crate::models::NonPartneredSmallParcelPackageOutput>) -> NonPartneredSmallParcelDataOutput {
        NonPartneredSmallParcelDataOutput {
            package_list,
        }
    }
}


