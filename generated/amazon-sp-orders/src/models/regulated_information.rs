/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RegulatedInformation : The regulated information collected during purchase and used to verify the order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegulatedInformation {
    /// A list of regulated information fields as collected from the regulatory form.
    #[serde(default, rename = "Fields")]
    pub fields: Vec<crate::models::RegulatedInformationField>,
}

impl RegulatedInformation {
    /// The regulated information collected during purchase and used to verify the order.
    pub fn new(fields: Vec<crate::models::RegulatedInformationField>) -> RegulatedInformation {
        RegulatedInformation {
            fields,
        }
    }
}


