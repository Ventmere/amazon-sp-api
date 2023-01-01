/*
 * Selling Partner API for Vendor Direct Fulfillment Sandbox Test Data
 *
 * The Selling Partner API for Vendor Direct Fulfillment Sandbox Test Data provides programmatic access to vendor direct fulfillment sandbox test data.
 *
 * The version of the OpenAPI document: 2021-10-28
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TestCaseData : The set of test case data returned in response to the test data request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TestCaseData {
    /// Set of use cases that describes the possible test scenarios.
    #[serde(default, rename = "scenarios", skip_serializing_if = "Option::is_none")]
    pub scenarios: Option<Vec<crate::models::Scenario>>,
}

impl TestCaseData {
    /// The set of test case data returned in response to the test data request.
    pub fn new() -> TestCaseData {
        TestCaseData {
            scenarios: None,
        }
    }
}


