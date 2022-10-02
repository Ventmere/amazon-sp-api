/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContentModule : An A+ Content module. An A+ Content document is composed of content modules. The contentModuleType property selects which content module types to use.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContentModule {
    #[serde(rename = "contentModuleType")]
    pub content_module_type: crate::models::ContentModuleType,
    #[serde(rename = "standardCompanyLogo", skip_serializing_if = "Option::is_none")]
    pub standard_company_logo: Option<Box<crate::models::StandardCompanyLogoModule>>,
    #[serde(rename = "standardComparisonTable", skip_serializing_if = "Option::is_none")]
    pub standard_comparison_table: Option<Box<crate::models::StandardComparisonTableModule>>,
    #[serde(rename = "standardFourImageText", skip_serializing_if = "Option::is_none")]
    pub standard_four_image_text: Option<Box<crate::models::StandardFourImageTextModule>>,
    #[serde(rename = "standardFourImageTextQuadrant", skip_serializing_if = "Option::is_none")]
    pub standard_four_image_text_quadrant: Option<Box<crate::models::StandardFourImageTextQuadrantModule>>,
    #[serde(rename = "standardHeaderImageText", skip_serializing_if = "Option::is_none")]
    pub standard_header_image_text: Option<Box<crate::models::StandardHeaderImageTextModule>>,
    #[serde(rename = "standardImageSidebar", skip_serializing_if = "Option::is_none")]
    pub standard_image_sidebar: Option<Box<crate::models::StandardImageSidebarModule>>,
    #[serde(rename = "standardImageTextOverlay", skip_serializing_if = "Option::is_none")]
    pub standard_image_text_overlay: Option<Box<crate::models::StandardImageTextOverlayModule>>,
    #[serde(rename = "standardMultipleImageText", skip_serializing_if = "Option::is_none")]
    pub standard_multiple_image_text: Option<Box<crate::models::StandardMultipleImageTextModule>>,
    #[serde(rename = "standardProductDescription", skip_serializing_if = "Option::is_none")]
    pub standard_product_description: Option<Box<crate::models::StandardProductDescriptionModule>>,
    #[serde(rename = "standardSingleImageHighlights", skip_serializing_if = "Option::is_none")]
    pub standard_single_image_highlights: Option<Box<crate::models::StandardSingleImageHighlightsModule>>,
    #[serde(rename = "standardSingleImageSpecsDetail", skip_serializing_if = "Option::is_none")]
    pub standard_single_image_specs_detail: Option<Box<crate::models::StandardSingleImageSpecsDetailModule>>,
    #[serde(rename = "standardSingleSideImage", skip_serializing_if = "Option::is_none")]
    pub standard_single_side_image: Option<Box<crate::models::StandardSingleSideImageModule>>,
    #[serde(rename = "standardTechSpecs", skip_serializing_if = "Option::is_none")]
    pub standard_tech_specs: Option<Box<crate::models::StandardTechSpecsModule>>,
    #[serde(rename = "standardText", skip_serializing_if = "Option::is_none")]
    pub standard_text: Option<Box<crate::models::StandardTextModule>>,
    #[serde(rename = "standardThreeImageText", skip_serializing_if = "Option::is_none")]
    pub standard_three_image_text: Option<Box<crate::models::StandardThreeImageTextModule>>,
}

impl ContentModule {
    /// An A+ Content module. An A+ Content document is composed of content modules. The contentModuleType property selects which content module types to use.
    pub fn new(content_module_type: crate::models::ContentModuleType) -> ContentModule {
        ContentModule {
            content_module_type,
            standard_company_logo: None,
            standard_comparison_table: None,
            standard_four_image_text: None,
            standard_four_image_text_quadrant: None,
            standard_header_image_text: None,
            standard_image_sidebar: None,
            standard_image_text_overlay: None,
            standard_multiple_image_text: None,
            standard_product_description: None,
            standard_single_image_highlights: None,
            standard_single_image_specs_detail: None,
            standard_single_side_image: None,
            standard_tech_specs: None,
            standard_text: None,
            standard_three_image_text: None,
        }
    }
}


