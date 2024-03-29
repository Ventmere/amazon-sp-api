/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ImageOffsets : The top left corner of the cropped image, specified in the original image's coordinate space.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImageOffsets {
    #[serde(default, rename = "x")]
    pub x: Box<crate::models::IntegerWithUnits>,
    #[serde(default, rename = "y")]
    pub y: Box<crate::models::IntegerWithUnits>,
}

impl ImageOffsets {
    /// The top left corner of the cropped image, specified in the original image's coordinate space.
    pub fn new(x: crate::models::IntegerWithUnits, y: crate::models::IntegerWithUnits) -> ImageOffsets {
        ImageOffsets {
            x: Box::new(x),
            y: Box::new(y),
        }
    }
}


