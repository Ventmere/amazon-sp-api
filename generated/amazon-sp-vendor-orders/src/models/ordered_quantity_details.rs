/*
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderedQuantityDetails : Details of item quantity ordered



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderedQuantityDetails {
    /// The date when the line item quantity was updated by buyer. Must be in ISO-8601 date/time format.
    #[serde(default, rename = "updatedDate", skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<String>,
    #[serde(default, rename = "orderedQuantity", skip_serializing_if = "Option::is_none")]
    pub ordered_quantity: Option<Box<crate::models::ItemQuantity>>,
    #[serde(default, rename = "cancelledQuantity", skip_serializing_if = "Option::is_none")]
    pub cancelled_quantity: Option<Box<crate::models::ItemQuantity>>,
}

impl OrderedQuantityDetails {
    /// Details of item quantity ordered
    pub fn new() -> OrderedQuantityDetails {
        OrderedQuantityDetails {
            updated_date: None,
            ordered_quantity: None,
            cancelled_quantity: None,
        }
    }
}


