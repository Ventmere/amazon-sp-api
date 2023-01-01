/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ImageDimensions : The dimensions extending from the top left corner of the cropped image, or the top left corner of the original image if there is no cropping. Only `pixels` is allowed as the units value for ImageDimensions.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImageDimensions {
    #[serde(default, rename = "width")]
    pub width: Box<crate::models::IntegerWithUnits>,
    #[serde(default, rename = "height")]
    pub height: Box<crate::models::IntegerWithUnits>,
}

impl ImageDimensions {
    /// The dimensions extending from the top left corner of the cropped image, or the top left corner of the original image if there is no cropping. Only `pixels` is allowed as the units value for ImageDimensions.
    pub fn new(width: crate::models::IntegerWithUnits, height: crate::models::IntegerWithUnits) -> ImageDimensions {
        ImageDimensions {
            width: Box::new(width),
            height: Box::new(height),
        }
    }
}


