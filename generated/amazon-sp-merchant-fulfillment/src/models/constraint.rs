/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Constraint : A validation constraint.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Constraint {
    /// A regular expression.
    #[serde(rename = "ValidationRegEx", skip_serializing_if = "Option::is_none")]
    pub validation_reg_ex: Option<String>,
    /// A validation string.
    #[serde(rename = "ValidationString")]
    pub validation_string: String,
}

impl Constraint {
    /// A validation constraint.
    pub fn new(validation_string: String) -> Constraint {
        Constraint {
            validation_reg_ex: None,
            validation_string,
        }
    }
}

