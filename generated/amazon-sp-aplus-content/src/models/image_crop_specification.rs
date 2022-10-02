/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ImageCropSpecification : The instructions for optionally cropping an image. If no cropping is desired, set the dimensions to the original image size. If the image is cropped and no offset values are provided, then the coordinates of the top left corner of the cropped image, relative to the original image, are defaulted to (0,0).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImageCropSpecification {
    #[serde(rename = "size")]
    pub size: Box<crate::models::ImageDimensions>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<Box<crate::models::ImageOffsets>>,
}

impl ImageCropSpecification {
    /// The instructions for optionally cropping an image. If no cropping is desired, set the dimensions to the original image size. If the image is cropped and no offset values are provided, then the coordinates of the top left corner of the cropped image, relative to the original image, are defaulted to (0,0).
    pub fn new(size: crate::models::ImageDimensions) -> ImageCropSpecification {
        ImageCropSpecification {
            size: Box::new(size),
            offset: None,
        }
    }
}

