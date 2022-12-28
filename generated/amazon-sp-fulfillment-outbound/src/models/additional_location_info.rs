/*
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * The version of the OpenAPI document: 2020-07-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AdditionalLocationInfo : Additional location information.

/// Additional location information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AdditionalLocationInfo {
    #[serde(rename = "AS_INSTRUCTED")]
    ASINSTRUCTED,
    #[serde(rename = "CARPORT")]
    CARPORT,
    #[serde(rename = "CUSTOMER_PICKUP")]
    CUSTOMERPICKUP,
    #[serde(rename = "DECK")]
    DECK,
    #[serde(rename = "DOOR_PERSON")]
    DOORPERSON,
    #[serde(rename = "FRONT_DESK")]
    FRONTDESK,
    #[serde(rename = "FRONT_DOOR")]
    FRONTDOOR,
    #[serde(rename = "GARAGE")]
    GARAGE,
    #[serde(rename = "GUARD")]
    GUARD,
    #[serde(rename = "MAIL_ROOM")]
    MAILROOM,
    #[serde(rename = "MAIL_SLOT")]
    MAILSLOT,
    #[serde(rename = "MAILBOX")]
    MAILBOX,
    #[serde(rename = "MC_BOY")]
    MCBOY,
    #[serde(rename = "MC_GIRL")]
    MCGIRL,
    #[serde(rename = "MC_MAN")]
    MCMAN,
    #[serde(rename = "MC_WOMAN")]
    MCWOMAN,
    #[serde(rename = "NEIGHBOR")]
    NEIGHBOR,
    #[serde(rename = "OFFICE")]
    OFFICE,
    #[serde(rename = "OUTBUILDING")]
    OUTBUILDING,
    #[serde(rename = "PATIO")]
    PATIO,
    #[serde(rename = "PORCH")]
    PORCH,
    #[serde(rename = "REAR_DOOR")]
    REARDOOR,
    #[serde(rename = "RECEPTIONIST")]
    RECEPTIONIST,
    #[serde(rename = "RECEIVER")]
    RECEIVER,
    #[serde(rename = "SECURE_LOCATION")]
    SECURELOCATION,
    #[serde(rename = "SIDE_DOOR")]
    SIDEDOOR,

}

impl ToString for AdditionalLocationInfo {
    fn to_string(&self) -> String {
        match self {
            Self::ASINSTRUCTED => String::from("AS_INSTRUCTED"),
            Self::CARPORT => String::from("CARPORT"),
            Self::CUSTOMERPICKUP => String::from("CUSTOMER_PICKUP"),
            Self::DECK => String::from("DECK"),
            Self::DOORPERSON => String::from("DOOR_PERSON"),
            Self::FRONTDESK => String::from("FRONT_DESK"),
            Self::FRONTDOOR => String::from("FRONT_DOOR"),
            Self::GARAGE => String::from("GARAGE"),
            Self::GUARD => String::from("GUARD"),
            Self::MAILROOM => String::from("MAIL_ROOM"),
            Self::MAILSLOT => String::from("MAIL_SLOT"),
            Self::MAILBOX => String::from("MAILBOX"),
            Self::MCBOY => String::from("MC_BOY"),
            Self::MCGIRL => String::from("MC_GIRL"),
            Self::MCMAN => String::from("MC_MAN"),
            Self::MCWOMAN => String::from("MC_WOMAN"),
            Self::NEIGHBOR => String::from("NEIGHBOR"),
            Self::OFFICE => String::from("OFFICE"),
            Self::OUTBUILDING => String::from("OUTBUILDING"),
            Self::PATIO => String::from("PATIO"),
            Self::PORCH => String::from("PORCH"),
            Self::REARDOOR => String::from("REAR_DOOR"),
            Self::RECEPTIONIST => String::from("RECEPTIONIST"),
            Self::RECEIVER => String::from("RECEIVER"),
            Self::SECURELOCATION => String::from("SECURE_LOCATION"),
            Self::SIDEDOOR => String::from("SIDE_DOOR"),
        }
    }
}

impl Default for AdditionalLocationInfo {
    fn default() -> AdditionalLocationInfo {
        Self::ASINSTRUCTED
    }
}



