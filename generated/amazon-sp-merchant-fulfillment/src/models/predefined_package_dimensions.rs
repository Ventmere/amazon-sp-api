/*
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PredefinedPackageDimensions : An enumeration of predefined parcel tokens. If you specify a PredefinedPackageDimensions token, you are not obligated to use a branded package from a carrier. For example, if you specify the FedEx_Box_10kg token, you do not have to use that particular package from FedEx. You are only obligated to use a box that matches the dimensions specified by the token.  Note: Please note that carriers can have restrictions on the type of package allowed for certain ship methods. Check the carrier website for all details. Example: Flat rate pricing is available when materials are sent by USPS in a USPS-produced Flat Rate Envelope or Box.

/// An enumeration of predefined parcel tokens. If you specify a PredefinedPackageDimensions token, you are not obligated to use a branded package from a carrier. For example, if you specify the FedEx_Box_10kg token, you do not have to use that particular package from FedEx. You are only obligated to use a box that matches the dimensions specified by the token.  Note: Please note that carriers can have restrictions on the type of package allowed for certain ship methods. Check the carrier website for all details. Example: Flat rate pricing is available when materials are sent by USPS in a USPS-produced Flat Rate Envelope or Box.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PredefinedPackageDimensions {
    #[serde(rename = "FedEx_Box_10kg")]
    FedExBox10kg,
    #[serde(rename = "FedEx_Box_25kg")]
    FedExBox25kg,
    #[serde(rename = "FedEx_Box_Extra_Large_1")]
    FedExBoxExtraLarge1,
    #[serde(rename = "FedEx_Box_Extra_Large_2")]
    FedExBoxExtraLarge2,
    #[serde(rename = "FedEx_Box_Large_1")]
    FedExBoxLarge1,
    #[serde(rename = "FedEx_Box_Large_2")]
    FedExBoxLarge2,
    #[serde(rename = "FedEx_Box_Medium_1")]
    FedExBoxMedium1,
    #[serde(rename = "FedEx_Box_Medium_2")]
    FedExBoxMedium2,
    #[serde(rename = "FedEx_Box_Small_1")]
    FedExBoxSmall1,
    #[serde(rename = "FedEx_Box_Small_2")]
    FedExBoxSmall2,
    #[serde(rename = "FedEx_Envelope")]
    FedExEnvelope,
    #[serde(rename = "FedEx_Padded_Pak")]
    FedExPaddedPak,
    #[serde(rename = "FedEx_Pak_1")]
    FedExPak1,
    #[serde(rename = "FedEx_Pak_2")]
    FedExPak2,
    #[serde(rename = "FedEx_Tube")]
    FedExTube,
    #[serde(rename = "FedEx_XL_Pak")]
    FedExXLPak,
    #[serde(rename = "UPS_Box_10kg")]
    UPSBox10kg,
    #[serde(rename = "UPS_Box_25kg")]
    UPSBox25kg,
    #[serde(rename = "UPS_Express_Box")]
    UPSExpressBox,
    #[serde(rename = "UPS_Express_Box_Large")]
    UPSExpressBoxLarge,
    #[serde(rename = "UPS_Express_Box_Medium")]
    UPSExpressBoxMedium,
    #[serde(rename = "UPS_Express_Box_Small")]
    UPSExpressBoxSmall,
    #[serde(rename = "UPS_Express_Envelope")]
    UPSExpressEnvelope,
    #[serde(rename = "UPS_Express_Hard_Pak")]
    UPSExpressHardPak,
    #[serde(rename = "UPS_Express_Legal_Envelope")]
    UPSExpressLegalEnvelope,
    #[serde(rename = "UPS_Express_Pak")]
    UPSExpressPak,
    #[serde(rename = "UPS_Express_Tube")]
    UPSExpressTube,
    #[serde(rename = "UPS_Laboratory_Pak")]
    UPSLaboratoryPak,
    #[serde(rename = "UPS_Pad_Pak")]
    UPSPadPak,
    #[serde(rename = "UPS_Pallet")]
    UPSPallet,
    #[serde(rename = "USPS_Card")]
    USPSCard,
    #[serde(rename = "USPS_Flat")]
    USPSFlat,
    #[serde(rename = "USPS_FlatRateCardboardEnvelope")]
    USPSFlatRateCardboardEnvelope,
    #[serde(rename = "USPS_FlatRateEnvelope")]
    USPSFlatRateEnvelope,
    #[serde(rename = "USPS_FlatRateGiftCardEnvelope")]
    USPSFlatRateGiftCardEnvelope,
    #[serde(rename = "USPS_FlatRateLegalEnvelope")]
    USPSFlatRateLegalEnvelope,
    #[serde(rename = "USPS_FlatRatePaddedEnvelope")]
    USPSFlatRatePaddedEnvelope,
    #[serde(rename = "USPS_FlatRateWindowEnvelope")]
    USPSFlatRateWindowEnvelope,
    #[serde(rename = "USPS_LargeFlatRateBoardGameBox")]
    USPSLargeFlatRateBoardGameBox,
    #[serde(rename = "USPS_LargeFlatRateBox")]
    USPSLargeFlatRateBox,
    #[serde(rename = "USPS_Letter")]
    USPSLetter,
    #[serde(rename = "USPS_MediumFlatRateBox1")]
    USPSMediumFlatRateBox1,
    #[serde(rename = "USPS_MediumFlatRateBox2")]
    USPSMediumFlatRateBox2,
    #[serde(rename = "USPS_RegionalRateBoxA1")]
    USPSRegionalRateBoxA1,
    #[serde(rename = "USPS_RegionalRateBoxA2")]
    USPSRegionalRateBoxA2,
    #[serde(rename = "USPS_RegionalRateBoxB1")]
    USPSRegionalRateBoxB1,
    #[serde(rename = "USPS_RegionalRateBoxB2")]
    USPSRegionalRateBoxB2,
    #[serde(rename = "USPS_RegionalRateBoxC")]
    USPSRegionalRateBoxC,
    #[serde(rename = "USPS_SmallFlatRateBox")]
    USPSSmallFlatRateBox,
    #[serde(rename = "USPS_SmallFlatRateEnvelope")]
    USPSSmallFlatRateEnvelope,

}

impl ToString for PredefinedPackageDimensions {
    fn to_string(&self) -> String {
        match self {
            Self::FedExBox10kg => String::from("FedEx_Box_10kg"),
            Self::FedExBox25kg => String::from("FedEx_Box_25kg"),
            Self::FedExBoxExtraLarge1 => String::from("FedEx_Box_Extra_Large_1"),
            Self::FedExBoxExtraLarge2 => String::from("FedEx_Box_Extra_Large_2"),
            Self::FedExBoxLarge1 => String::from("FedEx_Box_Large_1"),
            Self::FedExBoxLarge2 => String::from("FedEx_Box_Large_2"),
            Self::FedExBoxMedium1 => String::from("FedEx_Box_Medium_1"),
            Self::FedExBoxMedium2 => String::from("FedEx_Box_Medium_2"),
            Self::FedExBoxSmall1 => String::from("FedEx_Box_Small_1"),
            Self::FedExBoxSmall2 => String::from("FedEx_Box_Small_2"),
            Self::FedExEnvelope => String::from("FedEx_Envelope"),
            Self::FedExPaddedPak => String::from("FedEx_Padded_Pak"),
            Self::FedExPak1 => String::from("FedEx_Pak_1"),
            Self::FedExPak2 => String::from("FedEx_Pak_2"),
            Self::FedExTube => String::from("FedEx_Tube"),
            Self::FedExXLPak => String::from("FedEx_XL_Pak"),
            Self::UPSBox10kg => String::from("UPS_Box_10kg"),
            Self::UPSBox25kg => String::from("UPS_Box_25kg"),
            Self::UPSExpressBox => String::from("UPS_Express_Box"),
            Self::UPSExpressBoxLarge => String::from("UPS_Express_Box_Large"),
            Self::UPSExpressBoxMedium => String::from("UPS_Express_Box_Medium"),
            Self::UPSExpressBoxSmall => String::from("UPS_Express_Box_Small"),
            Self::UPSExpressEnvelope => String::from("UPS_Express_Envelope"),
            Self::UPSExpressHardPak => String::from("UPS_Express_Hard_Pak"),
            Self::UPSExpressLegalEnvelope => String::from("UPS_Express_Legal_Envelope"),
            Self::UPSExpressPak => String::from("UPS_Express_Pak"),
            Self::UPSExpressTube => String::from("UPS_Express_Tube"),
            Self::UPSLaboratoryPak => String::from("UPS_Laboratory_Pak"),
            Self::UPSPadPak => String::from("UPS_Pad_Pak"),
            Self::UPSPallet => String::from("UPS_Pallet"),
            Self::USPSCard => String::from("USPS_Card"),
            Self::USPSFlat => String::from("USPS_Flat"),
            Self::USPSFlatRateCardboardEnvelope => String::from("USPS_FlatRateCardboardEnvelope"),
            Self::USPSFlatRateEnvelope => String::from("USPS_FlatRateEnvelope"),
            Self::USPSFlatRateGiftCardEnvelope => String::from("USPS_FlatRateGiftCardEnvelope"),
            Self::USPSFlatRateLegalEnvelope => String::from("USPS_FlatRateLegalEnvelope"),
            Self::USPSFlatRatePaddedEnvelope => String::from("USPS_FlatRatePaddedEnvelope"),
            Self::USPSFlatRateWindowEnvelope => String::from("USPS_FlatRateWindowEnvelope"),
            Self::USPSLargeFlatRateBoardGameBox => String::from("USPS_LargeFlatRateBoardGameBox"),
            Self::USPSLargeFlatRateBox => String::from("USPS_LargeFlatRateBox"),
            Self::USPSLetter => String::from("USPS_Letter"),
            Self::USPSMediumFlatRateBox1 => String::from("USPS_MediumFlatRateBox1"),
            Self::USPSMediumFlatRateBox2 => String::from("USPS_MediumFlatRateBox2"),
            Self::USPSRegionalRateBoxA1 => String::from("USPS_RegionalRateBoxA1"),
            Self::USPSRegionalRateBoxA2 => String::from("USPS_RegionalRateBoxA2"),
            Self::USPSRegionalRateBoxB1 => String::from("USPS_RegionalRateBoxB1"),
            Self::USPSRegionalRateBoxB2 => String::from("USPS_RegionalRateBoxB2"),
            Self::USPSRegionalRateBoxC => String::from("USPS_RegionalRateBoxC"),
            Self::USPSSmallFlatRateBox => String::from("USPS_SmallFlatRateBox"),
            Self::USPSSmallFlatRateEnvelope => String::from("USPS_SmallFlatRateEnvelope"),
        }
    }
}

impl Default for PredefinedPackageDimensions {
    fn default() -> PredefinedPackageDimensions {
        Self::FedExBox10kg
    }
}




