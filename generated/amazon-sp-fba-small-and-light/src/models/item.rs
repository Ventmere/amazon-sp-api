/*
 * Selling Partner API for FBA Small And Light
 *
 * The Selling Partner API for FBA Small and Light lets you help sellers manage their listings in the Small and Light program. The program reduces the cost of fulfilling orders for small and lightweight FBA inventory. You can enroll or remove items from the program and check item eligibility and enrollment status. You can also preview the estimated program fees charged to a seller for items sold while enrolled in the program.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Item : An item to be sold.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Item {
    /// The Amazon Standard Identification Number (ASIN) value used to identify the item.
    #[serde(default, rename = "asin")]
    pub asin: String,
    #[serde(default, rename = "price")]
    pub price: Box<crate::models::MoneyType>,
}

impl Item {
    /// An item to be sold.
    pub fn new(asin: String, price: crate::models::MoneyType) -> Item {
        Item {
            asin,
            price: Box::new(price),
        }
    }
}


