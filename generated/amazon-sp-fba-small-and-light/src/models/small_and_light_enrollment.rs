/*
 * Selling Partner API for FBA Small And Light
 *
 * The Selling Partner API for FBA Small and Light lets you help sellers manage their listings in the Small and Light program. The program reduces the cost of fulfilling orders for small and lightweight FBA inventory. You can enroll or remove items from the program and check item eligibility and enrollment status. You can also preview the estimated program fees charged to a seller for items sold while enrolled in the program.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SmallAndLightEnrollment : The Small and Light enrollment status of the item indicated by the specified seller SKU.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SmallAndLightEnrollment {
    /// A marketplace identifier.
    #[serde(default, rename = "marketplaceId")]
    pub marketplace_id: String,
    /// Identifies an item in the given marketplace. SellerSKU is qualified by the seller's SellerId, which is included with every operation that you submit.
    #[serde(default, rename = "sellerSKU")]
    pub seller_sku: String,
    #[serde(default, rename = "status")]
    pub status: crate::models::SmallAndLightEnrollmentStatus,
}

impl SmallAndLightEnrollment {
    /// The Small and Light enrollment status of the item indicated by the specified seller SKU.
    pub fn new(marketplace_id: String, seller_sku: String, status: crate::models::SmallAndLightEnrollmentStatus) -> SmallAndLightEnrollment {
        SmallAndLightEnrollment {
            marketplace_id,
            seller_sku,
            status,
        }
    }
}


