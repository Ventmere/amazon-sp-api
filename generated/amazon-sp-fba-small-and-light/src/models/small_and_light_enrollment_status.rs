/*
 * Selling Partner API for FBA Small And Light
 *
 * The Selling Partner API for FBA Small and Light lets you help sellers manage their listings in the Small and Light program. The program reduces the cost of fulfilling orders for small and lightweight FBA inventory. You can enroll or remove items from the program and check item eligibility and enrollment status. You can also preview the estimated program fees charged to a seller for items sold while enrolled in the program.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SmallAndLightEnrollmentStatus : The Small and Light enrollment status of the item.

/// The Small and Light enrollment status of the item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmallAndLightEnrollmentStatus {
    #[serde(rename = "ENROLLED")]
    ENROLLED,
    #[serde(rename = "NOT_ENROLLED")]
    NOTENROLLED,

}

impl ToString for SmallAndLightEnrollmentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::ENROLLED => String::from("ENROLLED"),
            Self::NOTENROLLED => String::from("NOT_ENROLLED"),
        }
    }
}

impl Default for SmallAndLightEnrollmentStatus {
    fn default() -> SmallAndLightEnrollmentStatus {
        Self::ENROLLED
    }
}




