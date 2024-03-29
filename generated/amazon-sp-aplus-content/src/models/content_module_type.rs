/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContentModuleType : The type of A+ Content module.

/// The type of A+ Content module.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentModuleType {
    #[serde(rename = "STANDARD_COMPANY_LOGO")]
    COMPANYLOGO,
    #[serde(rename = "STANDARD_COMPARISON_TABLE")]
    COMPARISONTABLE,
    #[serde(rename = "STANDARD_FOUR_IMAGE_TEXT")]
    FOURIMAGETEXT,
    #[serde(rename = "STANDARD_FOUR_IMAGE_TEXT_QUADRANT")]
    FOURIMAGETEXTQUADRANT,
    #[serde(rename = "STANDARD_HEADER_IMAGE_TEXT")]
    HEADERIMAGETEXT,
    #[serde(rename = "STANDARD_IMAGE_SIDEBAR")]
    IMAGESIDEBAR,
    #[serde(rename = "STANDARD_IMAGE_TEXT_OVERLAY")]
    IMAGETEXTOVERLAY,
    #[serde(rename = "STANDARD_MULTIPLE_IMAGE_TEXT")]
    MULTIPLEIMAGETEXT,
    #[serde(rename = "STANDARD_PRODUCT_DESCRIPTION")]
    PRODUCTDESCRIPTION,
    #[serde(rename = "STANDARD_SINGLE_IMAGE_HIGHLIGHTS")]
    SINGLEIMAGEHIGHLIGHTS,
    #[serde(rename = "STANDARD_SINGLE_IMAGE_SPECS_DETAIL")]
    SINGLEIMAGESPECSDETAIL,
    #[serde(rename = "STANDARD_SINGLE_SIDE_IMAGE")]
    SINGLESIDEIMAGE,
    #[serde(rename = "STANDARD_TECH_SPECS")]
    TECHSPECS,
    #[serde(rename = "STANDARD_TEXT")]
    TEXT,
    #[serde(rename = "STANDARD_THREE_IMAGE_TEXT")]
    THREEIMAGETEXT,

}

impl ToString for ContentModuleType {
    fn to_string(&self) -> String {
        match self {
            Self::COMPANYLOGO => String::from("STANDARD_COMPANY_LOGO"),
            Self::COMPARISONTABLE => String::from("STANDARD_COMPARISON_TABLE"),
            Self::FOURIMAGETEXT => String::from("STANDARD_FOUR_IMAGE_TEXT"),
            Self::FOURIMAGETEXTQUADRANT => String::from("STANDARD_FOUR_IMAGE_TEXT_QUADRANT"),
            Self::HEADERIMAGETEXT => String::from("STANDARD_HEADER_IMAGE_TEXT"),
            Self::IMAGESIDEBAR => String::from("STANDARD_IMAGE_SIDEBAR"),
            Self::IMAGETEXTOVERLAY => String::from("STANDARD_IMAGE_TEXT_OVERLAY"),
            Self::MULTIPLEIMAGETEXT => String::from("STANDARD_MULTIPLE_IMAGE_TEXT"),
            Self::PRODUCTDESCRIPTION => String::from("STANDARD_PRODUCT_DESCRIPTION"),
            Self::SINGLEIMAGEHIGHLIGHTS => String::from("STANDARD_SINGLE_IMAGE_HIGHLIGHTS"),
            Self::SINGLEIMAGESPECSDETAIL => String::from("STANDARD_SINGLE_IMAGE_SPECS_DETAIL"),
            Self::SINGLESIDEIMAGE => String::from("STANDARD_SINGLE_SIDE_IMAGE"),
            Self::TECHSPECS => String::from("STANDARD_TECH_SPECS"),
            Self::TEXT => String::from("STANDARD_TEXT"),
            Self::THREEIMAGETEXT => String::from("STANDARD_THREE_IMAGE_TEXT"),
        }
    }
}

impl Default for ContentModuleType {
    fn default() -> ContentModuleType {
        Self::COMPANYLOGO
    }
}




