/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PrepInstruction : Preparation instructions for shipping an item to Amazon's fulfillment network. For more information about preparing items for shipment to Amazon's fulfillment network, see the Seller Central Help for your marketplace.

/// Preparation instructions for shipping an item to Amazon's fulfillment network. For more information about preparing items for shipment to Amazon's fulfillment network, see the Seller Central Help for your marketplace.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PrepInstruction {
    #[serde(rename = "Polybagging")]
    Polybagging,
    #[serde(rename = "BubbleWrapping")]
    BubbleWrapping,
    #[serde(rename = "Taping")]
    Taping,
    #[serde(rename = "BlackShrinkWrapping")]
    BlackShrinkWrapping,
    #[serde(rename = "Labeling")]
    Labeling,
    #[serde(rename = "HangGarment")]
    HangGarment,

}

impl ToString for PrepInstruction {
    fn to_string(&self) -> String {
        match self {
            Self::Polybagging => String::from("Polybagging"),
            Self::BubbleWrapping => String::from("BubbleWrapping"),
            Self::Taping => String::from("Taping"),
            Self::BlackShrinkWrapping => String::from("BlackShrinkWrapping"),
            Self::Labeling => String::from("Labeling"),
            Self::HangGarment => String::from("HangGarment"),
        }
    }
}

impl Default for PrepInstruction {
    fn default() -> PrepInstruction {
        Self::Polybagging
    }
}




